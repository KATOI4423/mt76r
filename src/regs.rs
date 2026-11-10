//! regs.rs
//!
//! SPDX-License-Identifier: BSD-3-Clause-Clear
//! Copyright (C) 2020 MediaTek Inc.

use kernel::bits;

#[inline(always)]
const fn mt_infra(offset: u32) -> u32 {
	const MT_INFRA_CFG_BASE: u32 = 0xfe000;
	MT_INFRA_CFG_BASE + offset
}

pub(crate) const MT_HIF_REMAP_L1: u32 = mt_infra(0x24c);
pub(crate) const MT_HIF_REMAP_L1_MASK: u32     = bits::genmask_u16(0..=15) as u32;
pub(crate) const MT_HIF_REMAP_L1_OFFSET: u32   = bits::genmask_u16(0..=15) as u32;
pub(crate) const MT_HIF_REMAP_L1_BASE: u32     = bits::genmask_u32(16..=31);
pub(crate) const MT_HIF_REMAP_BASE_L1: u32     = 0x40000;

pub(crate) const MT_HW_CHIPID: u32 = 0x70010200;

pub(crate) struct Regs;
impl Regs {
	// 1 MiB
	pub(crate) const SIZE: usize = 0x100000;
}
