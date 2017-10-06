extern crate frchal;
use frchal::*;

pub struct AnalogInput {
    m_channel: i32,
    m_port: HAL_AnalogInputHandle,
//    m_accumulatorOffset: i64,
}

impl AnalogInput {
    const kAccumulatorModuleNumber: i32 = 1;
    const kAccumulatorNumChannels: i32 = 2;
    const kAccumulatorChannels: [i32; 2] = [0, 1];

    pub fn fromChannel(channel: i32) -> Self {
        // todo: bounds checking on channel
        let port = unsafe { HAL_GetPort(channel) };
        let mut status: i32 = 0;
        let analogPort = unsafe { HAL_InitializeAnalogInputPort(port, &mut status) };
        if status != 0 {
            // todo: error
        }
        return AnalogInput {
            m_channel: channel,
            m_port: analogPort
        };
    }
    pub fn getValue(&self) -> i32 {
        let mut status: i32 = 0;
        let value = unsafe { HAL_GetAnalogValue(self.m_port, &mut status) };
        // todo: check error status
        return value;
    }
    pub fn getVoltage(&self) -> f64 {
        let mut status: i32 = 0;
        let value = unsafe { HAL_GetAnalogVoltage(self.m_port, &mut status) };
        // todo: check error status
        return value;
    }
}

impl Drop for AnalogInput {
    fn drop(&mut self) {
        unsafe { HAL_FreeAnalogInputPort(self.m_port); }
        self.m_port = HAL_kInvalidHandle as i32;
    }
}
