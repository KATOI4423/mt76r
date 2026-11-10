// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

pub(crate) mod mmio;
pub(crate) mod pci;
pub(crate) mod regs;

kernel::module_pci_driver! {
	type: pci::Mt7921,
	name: "mt76r",
	authors: ["KATOI <katoi4423@gmail.com>"],
	description: "MediaTek MT7921E (PCIe) wireless driver by Rust",
	license: "Dual BSD/GPL",
}
