//! mt7921/pci.rs
//!
//! SPDX-License-Identifier: BSD-3-Clause-Clear
//! Copyright (C) 2020 MediaTec Inc.

use kernel::devres::Devres;
use kernel::pci;
use kernel::prelude::*;

use crate::mmio::Bar0;
use crate::regs;
use crate::mmio::Mt7921Io;

#[pin_data(PinnedDrop)]
pub(crate) struct Mt7921 {
	#[pin]
	bar: Devres<Bar0>,
	index: Mt7921Index,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Mt7921Index(u8);

impl Mt7921Index {
	const NO_EVENTFD: Self = Self(0);
}

kernel::pci_device_table!(
	PCI_TABLE,
	MODULE_PCI_TABLE,
	<Mt7921 as pci::Driver>::IdInfo,
	[
		(
			pci::DeviceId::from_id(pci::Vendor::MEDIATEK, 0x7922),
			Mt7921Index::NO_EVENTFD,
		),
		(
			pci::DeviceId::from_id(pci::Vendor::ITTIM /* 0x0b48 */, 0x7922),
			Mt7921Index::NO_EVENTFD,
		),
		(
			pci::DeviceId::from_id(pci::Vendor::MEDIATEK, 0x0616),
			Mt7921Index::NO_EVENTFD,
		),
	]
);

impl pci::Driver for Mt7921 {
	type IdInfo = Mt7921Index;

	const ID_TABLE: pci::IdTable<Self::IdInfo> = &PCI_TABLE;

	fn probe(pdev: &pci::Device<kernel::device::Core>, id_info: &Self::IdInfo) -> impl PinInit<Self, Error> {
		pin_init::pin_init_scope(move || {
			pr_info!("Mt7921 probed!\n");
			pr_info!("  vendor id: {}\n", &pdev.vendor_id());
			pr_info!("  device id: {}\n", &pdev.device_id());
			pr_info!("  id_info: {}\n", &id_info.0);

			pdev.enable_device_mem()?;
			pdev.set_master();

			Ok(try_pin_init!(Self {
				bar <- pdev.iomap_region_sized::<{ regs::Regs::SIZE }>(0, c"mt76r"),
				_: {
					let bar = bar.access(pdev.as_ref())?;
					let chipid = bar.l1_rr(regs::MT_HW_CHIPID)?;
					pr_info!("chip id: {:#x}\n", chipid);
				},
				index: *id_info,
			}))
		})
	}

	fn unbind(dev: &pci::Device<kernel::device::Core>, this: Pin<&Self>) {
		pr_info!("Mt7921 unbind!\n");
	}
}

#[pinned_drop]
impl PinnedDrop for Mt7921 {
	fn drop(self: Pin<&mut Self>) {
		pr_info!("Remove Mt7921\n");
	}
}
