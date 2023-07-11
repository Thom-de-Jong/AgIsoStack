
use core::time::Duration;

use alloc::collections::VecDeque;

use crate::hardware_integration::{TimeDriver, TimeDriverTrait, CanDriverTrait};
use crate::{CanMessage, CanNetworkManager, ParameterGroupNumber, CanPriority};

const ETP_TIMEOUT_T1: Duration = Duration::from_millis(750);
const ETP_TIMEOUT_T2: Duration = Duration::from_millis(1250);
const ETP_TIMEOUT_T3: Duration = Duration::from_millis(1750);
const ETP_TIMEOUT_T4: Duration = Duration::from_millis(1050);


pub struct ExtendedTransportProtocolManager {
    current_state: State,
    message_backlog: VecDeque<CanMessage>,
    message_to_send: Option<CanMessage>,
    next_timeout: Duration,
    // _receive_buffer: Vec<u8>,
    // receive_pgn: Option<PGN>,
    // _receive_nr_of_packets: u8,
}

impl ExtendedTransportProtocolManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send(&mut self, message: CanMessage) {
        // Start a new sending connection or if we are already in a connection, store the message in the backlog.
        if State::Idle == self.current_state {
            self.open_connection(message);
        } else {
            self.message_backlog.push_back(message);
        }
    }

    pub fn update<T: CanDriverTrait>(&mut self, network_manager: &mut CanNetworkManager<T>) -> Option<CanMessage> {
        // // If connected and messages are not received on time, send a timeout message and change state.
        // if let Some(message_to_send) = &self.message_to_send {
        //     if TimeDriver::time_elapsed() >= self.next_timeout && self.state() != State::Idle {
        //         // can.write(
        //         //     PDU::new_etp_connection_abort(
        //         //         EtpAbortReasons::Timeout,
        //         //         pdu_to_send.pgn(),
        //         //         pdu_to_send.destination_address(),
        //         //         claimed_address,
        //         //     )
        //         //     .into(),
        //         // );
        //         self.close_connection();
        //     }
        // }

        match self.current_state {
            State::Idle => {
                if let Some(message) = self.message_backlog.pop_front() {
                    self.open_connection(message);
                }
            }
            State::SendRequestToSend => {
                if let Some(message) = self.message_to_send {
                    let message = CanMessage::new(
                        CanPriority::PriorityLowest7,
                        ParameterGroupNumber::ExtendedTransportProtocolConnectionManagement,
                        self.internal_control_function.address(),
                        self.partnered_control_function.address(),
                        &data,
                    );
                    network_manager.send_can_message(message);

                    let mut data: [u8; 8] = [0xFF; 8];
                    data[0] = EtpCmControlByte::RequestToSend as u8;
                    data[1..=4].copy_from_slice(&(message.len() as u32).to_le_bytes());
                    data[5..=7].copy_from_slice(&message.pgn().as_bytes());
    
                    // network_manager.send_can_message(network_manager, &data);
    
                    self.set_state(State::WaitForGetMemoryResponse);
                }
                
                // let number_of_bytes = message.len() as u32;

                // can.write(
                //     PDU::new_etp_request_to_send(
                //         number_of_bytes,
                //         pdu.pgn(),
                //         pdu.destination_address(),
                //         pdu.source_address(),
                //     )
                //     .into(),
                // );
            }
            State::WaitForClearToSendResponse => todo!(),
            State::SendData => todo!(),
            State::SendClearToSend => todo!(),
        }

        None





        // Statements after this need to process a PDU.
        // let pdu = match pdu {
        //     Some(pdu) => pdu,
        //     None => return None,
        // };

        // // Received a request to send meant for us.
        // if pdu.is_etp_request_to_send() && pdu.is_address_specific(claimed_address) {
        //     // When idling, accept the request to send and send a clear to send message.
        //     if self.state() == State::Idle {
        //         //         let data: [u8; 8] = pdu.data::<8>();

        //         //         let nr_of_bytes: u16 = u16::from_le_bytes([data[1], data[2]]);
        //         //         self.receive_buffer = vec![0xFF; nr_of_bytes as usize];
        //         //         self.receive_nr_of_packets = u8::min(data[3], data[4]);
        //         //         let packet_pgn: PGN = PGN::from_le_bytes([data[5], data[6], data[7]]);

        //         //         can.write(PDU::new_tp_clear_to_send(
        //         //             self.receive_nr_of_packets,
        //         //             1,
        //         //             packet_pgn,
        //         //             pdu.source_address(),
        //         //             claimed_address,
        //         //         ).into());

        //         //         self.receive_pgn = Some(packet_pgn);
        //         //         self.timeout_time = time + TP_TIMEOUT_T2;
        //         //     } else {
        //         //         // If we are already in a connection, abort the new connection.
        //         //         can.write(PDU::new_tp_connection_abort(TpAbortReasons::AlreadyConnected, pdu.pgn(), pdu.source_address(), claimed_address).into());
        //     }
        // }

        // // Received a clear to send meant for us.
        // if pdu.is_etp_clear_to_send() && pdu.is_address_specific(claimed_address) {
        //     if self.state() == State::Sending {
        //         let data: [u8; 8] = pdu.data::<8>();
        //         let nr_of_packets = data[1];
        //         let next_packet = u32::from_le_bytes([data[2], data[3], data[4], 0x00]);

        //         if nr_of_packets == 0 {
        //             self.next_timeout = time + ETP_TIMEOUT_T4;
        //         } else {
        //             self.send_pdu_data(
        //                 can,
        //                 next_packet,
        //                 nr_of_packets,
        //                 pdu.source_address(),
        //                 claimed_address,
        //             );
        //             self.next_timeout = time + ETP_TIMEOUT_T3;
        //         }
        //     }
        // }

        // // Received an end of message meant for us.
        // if pdu.is_etp_end_of_message_acknowledge() && pdu.is_address_specific(claimed_address) {
        //     if self.state() == State::Sending {
        //         let finished_pdu = self.message_to_send.take();
        //         self.close_connection();

        //         // If we have PDUs in the backlog, start a new connection.
        //         if let Some(pdu) = self.backlog.pop_front() {
        //             self.open_connection(can, pdu, time);
        //         }

        //         return finished_pdu;
        //     }
        // }

        // // Received an data transfer meant for us.
        // if pdu.is_etp_data_transfer() && pdu.is_address_specific(claimed_address) {
        //     if self.state() == State::Receiving {
        //         let data: [u8; 8] = pdu.data::<8>();
        //         let packet_nr = data[0];

        //         for i in 0..7 {
        //             self.receive_buffer[((packet_nr-1)*7+i) as usize] = data[(i+1) as usize];
        //         }

        //         self.timeout_time = time + TP_TIMEOUT_T1;

        //         if self.receive_nr_of_packets == packet_nr {
        //             if let Some(pgn) = self.receive_pgn {
        //                 can.write(PDU::new_tp_end_of_message_acknowledge(self.receive_buffer.len() as u16, self.receive_nr_of_packets, pgn, pdu.source_address(), claimed_address).into());
        //             }
        //             self.close_connection();
        //         }
        //     }
        // }
    }

    pub fn process_can_message(&mut self, message: CanMessage) -> Option<CanMessage> {
        match message.pgn() {
            ParameterGroupNumber::ExtendedTransportProtocolConnectionManagement => {
                if let Ok(control_Byte) = message.get_u8_at(0).try_into() {
					match control_Byte {
                        EtpCmControlByte::RequestToSend => {

                        }
                        EtpCmControlByte::ClearToSend => {

                        },
                        EtpCmControlByte::DataPacketOffset => {

                        },
                        EtpCmControlByte::EndOfMessageAcknowledgement => {

                        },
                        EtpCmControlByte::ConnectionAbort => {

                        },
                    }
                }
            }
            ParameterGroupNumber::ExtendedTransportProtocolDataTransfer => {
                
            }
            _ => { }
        }
        None
    }


    fn open_connection(&mut self, message: CanMessage) {
        self.next_timeout = TimeDriver::time_elapsed() + ETP_TIMEOUT_T3;
        self.message_to_send = Some(message);
        self.set_state(State::SendRequestToSend);
    }

    fn close_connection(&mut self) {
        self.next_timeout = Duration::MAX;
        self.message_to_send = None;
        // self.receive_buffer.clear();
        // self.receive_pgn = None;
        // self.receive_nr_of_packets = 0;
    }

    fn send_pdu_data(
        &self,
        can: &mut Box<dyn CanDriverTrait>,
        next_packet: u32,
        number_of_packets: u8,
        da: IsobusAddress,
        sa: IsobusAddress,
    ) {
        if let Some(pdu_to_send) = &self.message_to_send {
            can.write(
                PDU::new_etp_data_packet_offset(
                    number_of_packets,
                    next_packet - 1,
                    pdu_to_send.pgn(),
                    da,
                    sa,
                )
                .into(),
            );

            let chunks: Vec<&[u8]> = pdu_to_send.data_raw().chunks(7).collect();
            for i in 0..number_of_packets {
                can.write(
                    PDU::new_etp_data_transfer(
                        i + 1,
                        chunks[(next_packet - 1 + i as u32) as usize],
                        da,
                        sa,
                    )
                    .into(),
                );
            }
        }
    }

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
            // _receive_buffer: Vec::new(),
            // receive_pgn: None,
            // _receive_nr_of_packets: 0,
        }
    }
}



#[derive(Debug, PartialEq)]
enum State {
    Idle,
    SendRequestToSend,
	WaitForClearToSendResponse,
    SendData,
    SendClearToSend,
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
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
