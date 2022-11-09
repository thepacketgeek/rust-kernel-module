//! Virtual Device Module
//!
//! Created from following this tutorial: `https://www.jackos.io/rust-kernel/rust-for-linux.html`

use kernel::prelude::*;

use kernel::file::{flags, File, Operations};
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::sync::smutex::Mutex;
use kernel::sync::{Arc, ArcBorrow};
use kernel::{miscdev, Module};

module! {
    type: VDev,
    name: "vdev",
    license: "GPL",
    params: {
        devices: u32 {
            // Set this with `insmod rust_vdev.ko devices=4`
            default: 1,
            permissions: 0o644,
            description: "Number of virtual devices",
        },
    },
}

struct VDev {
    _devs: Vec<Pin<Box<miscdev::Registration<VDev>>>>,
}

struct Device {
    number: usize,
    contents: Mutex<Vec<u8>>,
}

// https://rust-for-linux.github.io/docs/kernel/file/trait.Operations.html
#[vtable]
impl Operations for VDev {
    // The data that is passed into the open method
    type OpenData = Arc<Device>;
    // The data that is returned by running an open method
    type Data = Arc<Device>;

    fn open(context: &Self::OpenData, file: &File) -> Result<Self::Data> {
        pr_info!("File for device {} was opened\n", context.number);

        // Clear the data if file is opened in `write only` mode
        if file.flags() & flags::O_ACCMODE == flags::O_WRONLY {
            context.contents.lock().clear();
        }

        Ok(context.clone())
    }

    // Read the data contents and write them into the buffer provided
    fn read(
        data: ArcBorrow<'_, Device>,
        _file: &File,
        writer: &mut impl IoBufferWriter,
        offset: u64,
    ) -> Result<usize> {
        pr_info!("File for device {} was read\n", data.number);
        let offset = offset.try_into()?;
        let vec = data.contents.lock();
        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(offset));
        writer.write_slice(&vec[offset..][..len])?;
        Ok(len)
    }

    // Read from the buffer and write the data in the contents after locking the mutex
    fn write(
        data: ArcBorrow<'_, Device>,
        _file: &File,
        reader: &mut impl IoBufferReader,
        offset: u64,
    ) -> Result<usize> {
        pr_info!("File for device {} was written\n", data.number);
        let offset = offset.try_into()?;
        let len = reader.len();
        let new_len = len.checked_add(offset).ok_or(EINVAL)?;
        let mut vec = data.contents.lock();
        if new_len > vec.len() {
            vec.try_resize(new_len, 0)?;
        }
        reader.read_slice(&mut vec[offset..][..len])?;
        Ok(len)
    }
}

impl Module for VDev {
    fn init(_name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        let count = {
            let lock = module.kernel_param_lock();
            (*devices.read(&lock)).try_into()?
        };
        pr_info!("------------------------\n");
        pr_info!("starting {count} vdevices!\n");
        pr_info!("------------------------\n");
        let mut devs = Vec::try_with_capacity(count)?;
        for i in 0..count {
            let dev = Arc::try_new(Device {
                number: i,
                contents: Mutex::new(Vec::new()),
            })?;
            let reg = miscdev::Registration::new_pinned(fmt!("vdev{i}"), dev)?;
            devs.try_push(reg)?;
        }

        Ok(Self { _devs: devs })
    }
}

impl Drop for VDev {
    fn drop(&mut self) {
        pr_info!("------------------------\n");
        pr_info!("removing {count} vdevices!\n", count=self._devs.len());
        pr_info!("------------------------\n");
    }
}