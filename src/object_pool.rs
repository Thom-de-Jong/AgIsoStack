
use alloc::collections::VecDeque;

use crate::objects::*;


#[derive(Debug)]
pub struct ObjectPool<const N: usize> {
    objects: VecDeque<Object>,
    colour_map: [u8; 256],
    colour_palette: [Colour; 256],

    temp_size: Option<u32>,
}

impl<const N: usize> ObjectPool<N> {
    pub fn new() -> Self {
        // Setup the default colour map
        let mut colour_map: [u8; 256] = [0xFFu8; 256];
        for i in 0..=u8::MAX {
            colour_map[i as usize] = i;
        }

        ObjectPool {
            objects: VecDeque::new(),
            colour_map,
            colour_palette: Colour::COLOUR_PALETTE,

            temp_size: None,
        }
    }

    pub fn from_iop<I>(data: I) -> Result<Self, ()>
    where
        I: IntoIterator<Item = u8>,
    {
        //let mut data = data.into_iter();

        let mut op = Self::new();

        while let Ok(o) = Object::read(&mut data) {
            if op.objects.push_back(o).is_err() {
                log::error!("Object pool not big enough. Used size: {}", op.size());
                return Err(());
            }
        }

        Ok(op)
    }

    pub fn as_iop(&self, buffer: &[u8]) -> Vec<u8, N> {
        let mut data = Vec::new();

        for obj in &self.objects {
            buffer.extend(obj.write());
        }

        // self.temp_size = Some(data.len() as u32);

        data
    }

    pub fn add(&mut self, obj: Object) {
        if let Some(len) = self.temp_size {
            self.temp_size = Some(len + obj.write().len() as u32);
        }
        self.objects.push(obj);
    }

    pub fn size(&mut self) -> u32 {
        match self.temp_size {
            Some(len) => len,
            None => self.as_iop().len() as u32,
        }
    }

    pub fn object_by_id(&self, id: ObjectId) -> Option<&Object> {
        self.objects.iter().find(|&o| o.id() == id)
    }

    pub fn objects_by_type(&self, object_type: ObjectType) -> Vec<&Object> {
        self.objects
            .iter()
            .filter(|&o| o.object_type() == object_type)
            .collect()
    }

    // Get objects by type

    pub fn working_set_object(&self) -> Option<&WorkingSet> {
        match &self.objects_by_type(ObjectType::WorkingSet).first() {
            Some(Object::WorkingSet(o)) => Some(o),
            _ => None,
        }
    }

    pub fn data_mask_objects(&self) -> Vec<&DataMask> {
        let r: Vec<&DataMask> = self
            .objects_by_type(ObjectType::DataMask)
            .iter()
            .filter_map(|&o| match o {
                Object::DataMask(o) => Some(o),
                _ => None,
            })
            .collect();
        r
    }

    pub fn picture_graphic_objects(&self) -> Vec<&PictureGraphic> {
        let r: Vec<&PictureGraphic> = self
            .objects_by_type(ObjectType::PictureGraphic)
            .iter()
            .filter_map(|&o| match o {
                Object::PictureGraphic(o) => Some(o),
                _ => None,
            })
            .collect();
        r
    }

    // Get typed objects by id

    pub fn data_mask_object_by_id(&self, id: ObjectId) -> Option<&DataMask> {
        match &self.object_by_id(id) {
            Some(Object::DataMask(o)) => Some(o),
            _ => None,
        }
    }

    pub fn line_attributes_object_by_id(&self, id: ObjectId) -> Option<&LineAttributes> {
        match &self.object_by_id(id) {
            Some(Object::LineAttributes(o)) => Some(o),
            _ => None,
        }
    }

    pub fn color_by_index(&self, index: u8) -> Colour {
        self.colour_palette[self.colour_map[index as usize] as usize]
    }
}

impl<const N: usize> Iterator for ObjectPool<N> {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.objects.pop_front()
    }
}

impl<const N: usize> Default for ObjectPool<N> {
    fn default() -> Self {
        Self::new()
    }
}
