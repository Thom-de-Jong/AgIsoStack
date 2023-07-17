
use core::time::Duration;

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::can_message::CanMessageBuilder;
use crate::hardware_integration::{TimeDriver, TimeDriverTrait, CanDriverTrait};
use crate::{CanMessage, CanNetworkManager, ParameterGroupNumber, CanPriority, Address};


const ETP_TIMEOUT_T1: Duration = Duration::from_millis(750);
const ETP_TIMEOUT_T2: Duration = Duration::from_millis(1250);
const ETP_TIMEOUT_T3: Duration = Duration::from_millis(1250);
const ETP_TIMEOUT_T4: Duration = Duration::from_millis(1050);

const MAX_NUMBER_OF_PACKETS_TO_SEND: u8 = 32;

pub struct ExtendedTransportProtocolManager {
    current_state: State,
    message_backlog: VecDeque<CanMessage>,
    // message_to_send: Option<CanMessage>,
    next_timeout: Duration,

    receiving_message_builder: CanMessageBuilder,
    receiving_data_buffer: Vec<u8>,
    receiving_number_of_packets: u8,
    // receiving_next_packet_number: u32,

    // receiving_source_address: Option<Address>,
    // receiving_destination_address: Option<Address>,
    // receiving_pgn: Option<ParameterGroupNumber>,
    // receiving_data_buffer: Vec<u8>,
}

impl ExtendedTransportProtocolManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send(&mut self, message: CanMessage) {
        // Start a new sending connection or if we are already in a connection, store the message in the backlog.
        if State::WaitingForRequestToSend == self.current_state {
            self.open_connection(message);
        } else {
            self.message_backlog.push_back(message);
        }
    }

    pub fn update<T: CanDriverTrait>(&mut self, network_manager: &mut CanNetworkManager<T>) -> Option<CanMessage> {
        let mut result = None;

        match self.current_state {
            State::WaitingForRequestToSend => {
                if let Some(message) = self.message_backlog.pop_front() {
                    self.open_connection(message);
                }
            }
            State::SendRequestToSend(message_to_send) => {
                let mut data: [u8; 8] = [0xFF; 8];
                data[0] = EtpCmControlByte::RequestToSend as u8;
                data[1..=4].copy_from_slice(&(message_to_send.len() as u32).to_le_bytes());
                data[5..=7].copy_from_slice(&message_to_send.pgn().as_bytes());
    
                network_manager.send_can_message(CanMessage::new(
                    CanPriority::PriorityLowest7,
                    ParameterGroupNumber::ExtendedTransportProtocolConnectionManagement,
                    message_to_send.source_address(),
                    message_to_send.destination_address(),
                    &data,
                ));
    
                self.next_timeout = TimeDriver::time_elapsed() + ETP_TIMEOUT_T3;
                self.set_state(State::WaitForClearToSend(message_to_send));
            }
            State::WaitForClearToSend(_) => {
                if TimeDriver::time_elapsed() > self.next_timeout {
					log::error!("[ETP]: Wait For Clear To Send Timeout");
                    self.set_state(State::SendConnectionAbort(AbortReason::Timeout));
				}
            }
            State::SendClearToSend => {
                let message = self.receiving_message_builder.build();

                let mut data: [u8; 8] = [0xFF; 8];
                data[0] = EtpCmControlByte::ClearToSend as u8;
                data[1] = MAX_NUMBER_OF_PACKETS_TO_SEND;
                data[2..=4].copy_from_slice(&(self.receiving_data_buffer.len()+1).to_le_bytes()[..=2]);
                data[5..=7].copy_from_slice(&message.pgn().as_bytes());
    
                network_manager.send_can_message(CanMessage::new(
                    CanPriority::PriorityLowest7,
                    ParameterGroupNumber::ExtendedTransportProtocolConnectionManagement,
                    message.source_address(),
                    message.destination_address(),
                    &data,
                ));

                self.next_timeout = TimeDriver::time_elapsed() + ETP_TIMEOUT_T2;
                self.set_state(State::WaitForDataPacketOffset);
            }
            State::WaitForDataPacketOffset => {
                if TimeDriver::time_elapsed() > self.next_timeout {
					log::error!("[ETP]: Wait For Data Packet Offset Timeout");
                    self.set_state(State::SendConnectionAbort(AbortReason::Timeout));
				}
            }
            State::SendDataPacketOffset(message_to_send, number_of_packets_to_send, next_packet_number_to_send) => {
                let mut data: [u8; 8] = [0xFF; 8];
                data[0] = EtpCmControlByte::DataPacketOffset as u8;
                data[1] = number_of_packets_to_send;
                data[2..=4].copy_from_slice(&(next_packet_number_to_send-1 as u32).to_le_bytes());
                data[5..=7].copy_from_slice(&message_to_send.pgn().as_bytes());
    
                network_manager.send_can_message(CanMessage::new(
                    CanPriority::PriorityLowest7,
                    ParameterGroupNumber::ExtendedTransportProtocolConnectionManagement,
                    message_to_send.source_address(),
                    message_to_send.destination_address(),
                    &data,
                ));
    
                self.next_timeout = TimeDriver::time_elapsed() + ETP_TIMEOUT_T1;
                self.set_state(State::SendDataTransfer(message_to_send, 1, number_of_packets_to_send));
            }
            State::WaitForDataTransfer => {
                if TimeDriver::time_elapsed() > self.next_timeout {
					log::error!("[ETP]: Wait For Data Timeout");
                    self.set_state(State::SendConnectionAbort(AbortReason::Timeout));
				}
            }
            State::SendDataTransfer(message_to_send, sequence_number, number_of_packets_to_send) => {
                let mut data: [u8; 8] = [0xFF; 8];
                data[0] = sequence_number;

                // TODO: use real data
    
                network_manager.send_can_message(CanMessage::new(
                    CanPriority::PriorityLowest7,
                    ParameterGroupNumber::ExtendedTransportProtocolDataTransfer,
                    message_to_send.source_address(),
                    message_to_send.destination_address(),
                    &data,
                ));
    
                if sequence_number < number_of_packets_to_send {
                    self.next_timeout = TimeDriver::time_elapsed() + ETP_TIMEOUT_T1;
                    self.set_state(State::SendDataTransfer(message_to_send, sequence_number + 1, number_of_packets_to_send));
                } else {
                    self.next_timeout = TimeDriver::time_elapsed() + ETP_TIMEOUT_T3;
                    self.set_state(State::WaitForClearToSend(message_to_send));
                }
            }
            State::SendEndOfMessageAcknowledgement => {
                let message = self.receiving_message_builder.build();

                let mut data: [u8; 8] = [0xFF; 8];
                data[0] = EtpCmControlByte::ClearToSend as u8;
                data[1] = MAX_NUMBER_OF_PACKETS_TO_SEND;
                data[2..=4].copy_from_slice(&(self.receiving_data_buffer.len()+1).to_le_bytes()[..=2]);
                data[5..=7].copy_from_slice(&message.pgn().as_bytes());
    
                network_manager.send_can_message(CanMessage::new(
                    CanPriority::PriorityLowest7,
                    ParameterGroupNumber::ExtendedTransportProtocolConnectionManagement,
                    message.source_address(),
                    message.destination_address(),
                    &data,
                ));

                self.close_connection();

                result = Some(self.receiving_message_builder.data(&self.receiving_data_buffer).build())
            }
            
            State::SendConnectionAbort(abort_reason) => {
                self.abort_connection(abort_reason, network_manager);
            }
        }

        result
    }

    pub fn process_can_message(&mut self, message: &CanMessage) -> Option<CanMessage> {
        match message.pgn() {
            ParameterGroupNumber::ExtendedTransportProtocolConnectionManagement => {
                if let Ok(control_byte) = message.get_u8_at(0).try_into() {
					match control_byte {
                        EtpCmControlByte::RequestToSend => {
                            if State::WaitingForRequestToSend == self.current_state {
                                self.receiving_data_buffer = Vec::with_capacity(message.get_u32_at(1) as usize);
                                let pgn = message.get_pgn_at(5);

                                self.receiving_message_builder
                                    .pgn(pgn)
                                    .source_address(message.source_address())
                                    .destination_address(message.destination_address());

                                self.set_state(State::SendClearToSend)
                            } else {

                            }
                        }
                        EtpCmControlByte::ClearToSend => {
                            if let State::WaitForClearToSend(message_to_send) = self.current_state {
                                if message_to_send.pgn() != message.get_pgn_at(5) {
                                    self.set_state(State::SendConnectionAbort(AbortReason::BadClearToSendPgn));
                                    return None;
                                }

                                let number_of_packets_to_send = message.get_u8_at(1);
                                let next_packet_number_to_send = message.get_u24_at(2);

                                if number_of_packets_to_send == 0 {
                                    self.next_timeout = TimeDriver::time_elapsed() + ETP_TIMEOUT_T4;
                                    return None;
                                }

                                self.set_state(State::SendDataPacketOffset(
                                    message_to_send,
                                    number_of_packets_to_send,
                                    next_packet_number_to_send,
                                ))
                            }
                            None
                        }
                        EtpCmControlByte::DataPacketOffset => {
                            if State::WaitForDataPacketOffset == self.current_state {
                                // self.receiving_number_of_bytes = message.get_u32_at(1);
                                // let pgn = message.get_pgn_at(5);

                                // self.receiving_message_builder
                                //     .pgn(pgn)
                                //     .source_address(message.source_address())
                                //     .destination_address(message.destination_address());

                                self.receiving_number_of_packets = message.get_u8_at(1);
                                // self.receiving_next_packet_number = message.get_u24_at(2);

                                self.next_timeout = TimeDriver::time_elapsed() + ETP_TIMEOUT_T1;
                                self.set_state(State::WaitForDataTransfer)
                            }
                            None
                        }
                        EtpCmControlByte::EndOfMessageAcknowledgement => {
                            if let State::WaitForClearToSend(message_to_send) = self.current_state {
                                if message_to_send.pgn() != message.get_pgn_at(5) {
                                    self.set_state(State::SendConnectionAbort(AbortReason::Other));
                                    return None;
                                }

                                let number_of_bytes_transferred = message.get_u32_at(1);

                                if number_of_bytes_transferred != message_to_send.len() as u32 {
                                    self.set_state(State::SendConnectionAbort(AbortReason::Other));
                                    return None;
                                }

                                self.close_connection();
                            }
                            None
                        }
                        EtpCmControlByte::ConnectionAbort => {
                            None
                        }
                    }
                } else {
                    None
                }
            }
            ParameterGroupNumber::ExtendedTransportProtocolDataTransfer => {
                if State::WaitForDataTransfer == self.current_state {
                    let sequence_number = message.get_u8_at(0);
                    // let offset = self.receiving_next_packet_number + sequence_number as u32;

                    // TODO: implement end edge case, len < 7
                    self.receiving_data_buffer.extend_from_slice(&message.data()[1..=7]);

                    // Have we filled the entire buffer
                    if self.receiving_data_buffer.len() == self.receiving_data_buffer.capacity() {
                        self.set_state(State::SendEndOfMessageAcknowledgement);
                        self.next_timeout = TimeDriver::time_elapsed() + ETP_TIMEOUT_T1;
                    } else {
                        self.next_timeout = TimeDriver::time_elapsed() + ETP_TIMEOUT_T1;
                    }
                }
                None
            }
            _ => { None }
        }
    }


    fn open_connection(&mut self, message: CanMessage) {
        self.set_state(State::SendRequestToSend(message));
    }

    fn close_connection(&mut self) {
        self.next_timeout = Duration::MAX;
        // self.message_to_send = None;
        // self.receive_buffer.clear();
        // self.receive_pgn = None;
        // self.receive_nr_of_packets = 0;
        self.set_state(State::WaitingForRequestToSend);
    }

    fn abort_connection<T: CanDriverTrait>(&mut self, abort_reason: AbortReason, network_manager: &mut CanNetworkManager<T>) {
        let message = if self.message_to_send.is_some() {
            self.message_to_send.unwrap()
        } else {
            self.receiving_message_builder.build()
        };

        let mut data: [u8; 8] = [0xFF; 8];
        data[0] = EtpCmControlByte::ConnectionAbort as u8;
        data[1] = abort_reason as u8;
        data[5..=7].copy_from_slice(&message.pgn().as_bytes());
    
        network_manager.send_can_message(CanMessage::new(
            CanPriority::PriorityLowest7,
            ParameterGroupNumber::ExtendedTransportProtocolConnectionManagement,
            message.source_address(),
            message.destination_address(),
            &data,
        ));

        self.close_connection();
    }


    // fn send_data(
    //     &self,
    //     can: &mut Box<dyn CanDriverTrait>,
    //     next_packet: u32,
    //     number_of_packets: u8,
    //     da: IsobusAddress,
    //     sa: IsobusAddress,
    // ) {
    //     if let Some(pdu_to_send) = &self.message_to_send {
    //         can.write(
    //             PDU::new_etp_data_packet_offset(
    //                 number_of_packets,
    //                 next_packet - 1,
    //                 pdu_to_send.pgn(),
    //                 da,
    //                 sa,
    //             )
    //             .into(),
    //         );

    //         let chunks: Vec<&[u8]> = pdu_to_send.data_raw().chunks(7).collect();
    //         for i in 0..number_of_packets {
    //             can.write(
    //                 PDU::new_etp_data_transfer(
    //                     i + 1,
    //                     chunks[(next_packet - 1 + i as u32) as usize],
    //                     da,
    //                     sa,
    //                 )
    //                 .into(),
    //             );
    //         }
    //     }
    // }

    fn set_state(&mut self, state: State) {
        self.current_state = state;
    }
}

impl Default for ExtendedTransportProtocolManager {
    fn default() -> Self {
        Self {
            current_state: State::default(),
            message_backlog: VecDeque::new(),
            message_to_send: None,
            next_timeout: Duration::MAX,
            receiving_message_builder: CanMessage::builder(),
            receiving_number_of_bytes: 0,
            receiving_number_of_packets: 0,
            receiving_next_packet_number: 1,
            // _receive_buffer: Vec::new(),
            // receive_pgn: None,
            // _receive_nr_of_packets: 0,
        }
    }
}



#[derive(Debug, PartialEq)]
enum State {
    WaitingForRequestToSend,
    SendRequestToSend(CanMessage),
	WaitForClearToSend(CanMessage),
    SendClearToSend,
    WaitForDataPacketOffset,
    SendDataPacketOffset(CanMessage, u8, u32),
    WaitForDataTransfer,
    SendDataTransfer(CanMessage, u8, u8),
    SendEndOfMessageAcknowledgement,
    SendConnectionAbort(AbortReason),
}

impl Default for State {
    fn default() -> Self {
        Self::WaitingForRequestToSend
    }
}

/// Enumerates the multiplexor byte values for EtpCm Control Byte
#[repr(u8)]
#[derive(Debug, PartialEq)]
enum EtpCmControlByte {
    RequestToSend = 0x14,
    ClearToSend = 0x15,
    DataPacketOffset = 0x16,
    EndOfMessageAcknowledgement = 0x17,
    ConnectionAbort = 0xFF,
}

impl TryFrom<u8> for EtpCmControlByte {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x14 => Ok(Self::RequestToSend),
            0x15 => Ok(Self::ClearToSend),
            0x16 => Ok(Self::DataPacketOffset),
            0x17 => Ok(Self::EndOfMessageAcknowledgement),
            0xFF => Ok(Self::ConnectionAbort),
            _ => Err(()),
        }
    }
}


#[repr(u8)]
#[derive(Debug, PartialEq)]
enum AbortReason {
    AlreadyConnected = 0x01,                    //< Already in one or more connection-managed sessions and cannot support another
    Terminated = 0x02,                          //< System resources were needed for another task so this connection managed session was terminated
    Timeout = 0x03,                             //< A timeout occurred and this is the connection abort to close the session
    DataTransferInProgress = 0x04,              //< CTS messages received when data transfer is in progress
    RetransmitLimitReached = 0x05,              //< Maximum retransmit request limit reached
    UnexpectedDataTransfer = 0x06,              //< Unexpected data transfer packet
    BadSequenceNumber = 0x07,                   //< Bad sequence number (and software is not able to recover)
    DuplicateSequenceNumber = 0x08,             //< Duplicate sequence number (and software is not able to recover)
    UnexpectedDataPacketOffset = 0x09,          //< Unexpected EDPO packet
    BadDataPacketOffsetPgn = 0x0A,              //< Unexpected EDPO PGN (PGN in EDPO is bad)
    BadDataPacketOffsetNumberOfPackets = 0x0B,  //< EDPO number of packets is greater than CTS
    BadDataPacketOffsetOffset = 0x0C,           //< Bad EDPO offset
    BadClearToSendPgn = 0x0E,                   //< Unexpected ECTS PGN (PGN in ECTS is bad)
    //  = 0x0F,                                    //< ECTS requested packets exceeds message size
    Other = 0xFA,                               //< Any other reason
}

impl From<u8> for AbortReason {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::AlreadyConnected,
            0x02 => Self::Terminated,
            0x03 => Self::Timeout,
            0x04 => Self::DataTransferInProgress,
            0x05 => Self::RetransmitLimitReached,
            0x06 => Self::UnexpectedDataTransfer,
            0x07 => Self::BadSequenceNumber,
            0x08 => Self::DuplicateSequenceNumber,
            0x09 => Self::UnexpectedDataPacketOffset,
            0x0A => Self::BadDataPacketOffsetPgn,
            0x0B => Self::BadDataPacketOffsetNumberOfPackets,
            0x0C => Self::BadDataPacketOffsetOffset,
            _ => Self::Other,
        }
    }
}