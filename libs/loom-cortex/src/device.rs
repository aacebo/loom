use serde::{Deserialize, Serialize};
use tch::Device;

/// Serializable device specification
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CortexDevice {
    #[default]
    CudaIfAvailable,
    Cpu,
    Cuda(usize),
    Mps,
    Vulkan,
}

impl CortexDevice {
    pub fn is_cpu(&self) -> bool {
        matches!(self, Self::Cpu)
    }

    pub fn is_cuda(&self) -> bool {
        matches!(self, Self::Cuda(_) | Self::CudaIfAvailable)
    }

    pub fn is_mps(&self) -> bool {
        matches!(self, Self::Mps)
    }

    pub fn is_vulkan(&self) -> bool {
        matches!(self, Self::Vulkan)
    }

    pub fn is_gpu(&self) -> bool {
        self.is_cuda() || self.is_mps() || self.is_vulkan()
    }
}

impl From<CortexDevice> for Device {
    fn from(spec: CortexDevice) -> Self {
        match spec {
            CortexDevice::CudaIfAvailable => Self::cuda_if_available(),
            CortexDevice::Cpu => Self::Cpu,
            CortexDevice::Cuda(n) => Self::Cuda(n),
            CortexDevice::Mps => Self::Mps,
            CortexDevice::Vulkan => Self::Vulkan,
        }
    }
}

impl From<Device> for CortexDevice {
    fn from(device: Device) -> Self {
        match device {
            Device::Cpu => Self::Cpu,
            Device::Cuda(n) => Self::Cuda(n),
            Device::Mps => Self::Mps,
            Device::Vulkan => Self::Vulkan,
        }
    }
}
