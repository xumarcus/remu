use winit::{event::RawKeyEvent, keyboard::{KeyCode, PhysicalKey}};

use crate::backend::u4;

use super::Key;

pub(crate) struct Keypad([bool; 16]);

impl Default for Keypad {
    fn default() -> Self {
        Self([false; 16])
    }
}

impl Keypad {
    pub(crate) fn is_pressed(&self, x: u4) -> bool {
        self.0[x as usize]
    }

    pub(crate) fn get_pressed(&self) -> Option<Key> {
        let idx = self.0.iter().position(|&x| x)?;
        Some(idx as Key)
    }

    pub(crate) fn handle_key(&mut self, rke: RawKeyEvent) {
        let RawKeyEvent { physical_key, state } = rke;
        if let Some(x) = Self::from_pk(physical_key) {
            self.0[x as usize] = state.is_pressed();
        }
    }

    const fn from_pk(pk: PhysicalKey) -> Option<Key> {
        /*
        * 1->1 2->2 3->3 4->C
        * Q->4 W->5 E->6 R->D
        * A->7 S->8 D->9 F->E
        * Z->A X->0 C->B V->F
        */
        match pk {
            PhysicalKey::Code(KeyCode::Digit1) => Some(0x1),
            PhysicalKey::Code(KeyCode::Digit2) => Some(0x2),
            PhysicalKey::Code(KeyCode::Digit3) => Some(0x3),
            PhysicalKey::Code(KeyCode::Digit4) => Some(0xC),

            PhysicalKey::Code(KeyCode::KeyQ) => Some(0x4),
            PhysicalKey::Code(KeyCode::KeyW) => Some(0x5),
            PhysicalKey::Code(KeyCode::KeyE) => Some(0x6),
            PhysicalKey::Code(KeyCode::KeyR) => Some(0xD),

            PhysicalKey::Code(KeyCode::KeyA) => Some(0x7),
            PhysicalKey::Code(KeyCode::KeyS) => Some(0x8),
            PhysicalKey::Code(KeyCode::KeyD) => Some(0x9),
            PhysicalKey::Code(KeyCode::KeyF) => Some(0xE),

            PhysicalKey::Code(KeyCode::KeyZ) => Some(0xA),
            PhysicalKey::Code(KeyCode::KeyX) => Some(0x0),
            PhysicalKey::Code(KeyCode::KeyC) => Some(0xB),
            PhysicalKey::Code(KeyCode::KeyV) => Some(0xF),

            _ => None
        }
    }
}