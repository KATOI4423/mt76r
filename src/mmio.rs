//! mmio.rs
//!
//! SPDX-License-Identifier: BSD-3-Clause-Clear
//! Copyright (C) 2020 MediaTek Inc.

use kernel::io::Io;
use kernel::pci;
use kernel::prelude::*;

use crate::regs;

pub(crate) type Bar0 = pci::Bar<{ regs::Regs::SIZE }>;

const fn field_get(mask: u32, reg: u32) -> u32 {
	if mask == 0 {
		0
	} else {
		(reg & mask) >> mask.trailing_zeros()
	}
}

const fn field_prep(mask: u32, val: u32) -> u32 {
	if mask == 0 {
		val
	} else {
		val << mask.trailing_zeros()
	}
}

pub(crate) trait Mt7921Io {
	fn l1_rr(&self, address: u32) -> Result<u32>;
	fn rr(&self, offset: u32) -> Result<u32>;
	fn wr(&self, offset: u32, val: u32) -> Result<()>;
	fn rmw(&self, offset: u32, mask: u32, val: u32) -> Result<u32>;
	fn reg_map_l1(&self, address: u32) -> Result<u32>;
	fn rmw_field(&self, reg: u32, field: u32, val: u32) -> Result<u32>;
	fn reg_addr(&self, address: u32) -> Result<u32>;
}

impl Mt7921Io for Bar0 {
	fn l1_rr(&self, address: u32) -> Result<u32> {
		self.rr(self.reg_map_l1(address)?)
	}

	fn rr(&self, offset: u32) -> Result<u32> {
		let addr = self.reg_addr(offset)?.try_into()?;
		self.try_read32(addr)
	}

	fn wr(&self, offset: u32, val: u32) -> Result<()> {
		let addr = self.reg_addr(offset)?.try_into()?;
		self.try_write32(val, addr)
	}

	fn rmw(&self, offset: u32, mask: u32, val: u32) -> Result<u32> {
		let addr = self.reg_addr(offset)?;

		let val = val | (self.rr(addr)? & !mask);
		self.wr(addr, val)?;
		Ok(val)
	}

	fn reg_map_l1(&self, address: u32) -> Result<u32> {
		let offset = field_get(regs::MT_HIF_REMAP_L1_OFFSET, address);
		let base = field_get(regs::MT_HIF_REMAP_L1_BASE, address);

		self.rmw_field(regs::MT_HIF_REMAP_L1, regs::MT_HIF_REMAP_L1_MASK, base)?;
		self.rr(regs::MT_HIF_REMAP_L1)?;

		Ok(regs::MT_HIF_REMAP_BASE_L1 + offset)
	}

	#[inline(always)]
	fn rmw_field(&self, reg: u32, field: u32, val: u32) -> Result<u32> {
		self.rmw(reg, field, field_prep(field, val))
	}

	fn reg_addr(&self, address: u32) -> Result<u32> {
		if address < 0x100000 {
			return Ok(address);
		}

		for fixed in &Mt76ConnacRegMap::FIXED_MAP {
			match fixed.get_offset(address) {
				Some(addr) => return Ok(addr),
				None => continue,
			}
		}

		match address {
			0x18_000_000..0x18_c00_000
			 | 0x70_000_000..0x78_000_000
			 | 0x7c_000_000..0x7c_400_000
				=> self.reg_map_l1(address),
			_ => {
				pr_err!("Access currently unsupported address {address:0>8x}\n");
				Err(EFAULT)
			}
		}
	}
}

struct Mt76ConnacRegMap {
	pub(crate) phys: u32,
	pub(crate) maps: u32,
	pub(crate) size: u32,
}

impl Mt76ConnacRegMap {
	const FIXED_MAP: [Self; 44] = [
		Self { phys: 0x820d0000, maps: 0x30000, size: 0x10000 }, /* WF_LMAC_TOP (WF_WTBLON) */
		Self { phys: 0x820ed000, maps: 0x24800, size: 0x00800 }, /* WF_LMAC_TOP BN0 (WF_MIB) */
		Self { phys: 0x820e4000, maps: 0x21000, size: 0x00400 }, /* WF_LMAC_TOP BN0 (WF_TMAC) */
		Self { phys: 0x820e7000, maps: 0x21e00, size: 0x00200 }, /* WF_LMAC_TOP BN0 (WF_DMA) */
		Self { phys: 0x820eb000, maps: 0x24200, size: 0x00400 }, /* WF_LMAC_TOP BN0 (WF_LPON) */
		Self { phys: 0x820e2000, maps: 0x20800, size: 0x00400 }, /* WF_LMAC_TOP BN0 (WF_AGG) */
		Self { phys: 0x820e3000, maps: 0x20c00, size: 0x00400 }, /* WF_LMAC_TOP BN0 (WF_ARB) */
		Self { phys: 0x820e5000, maps: 0x21400, size: 0x00800 }, /* WF_LMAC_TOP BN0 (WF_RMAC) */
		Self { phys: 0x00400000, maps: 0x80000, size: 0x10000 }, /* WF_MCU_SYSRAM */
		Self { phys: 0x00410000, maps: 0x90000, size: 0x10000 }, /* WF_MCU_SYSRAM (configure register) */
		Self { phys: 0x40000000, maps: 0x70000, size: 0x10000 }, /* WF_UMAC_SYSRAM */
		Self { phys: 0x54000000, maps: 0x02000, size: 0x01000 }, /* WFDMA PCIE0 MCU DMA0 */
		Self { phys: 0x55000000, maps: 0x03000, size: 0x01000 }, /* WFDMA PCIE0 MCU DMA1 */
		Self { phys: 0x58000000, maps: 0x06000, size: 0x01000 }, /* WFDMA PCIE1 MCU DMA0 (MEM_DMA) */
		Self { phys: 0x59000000, maps: 0x07000, size: 0x01000 }, /* WFDMA PCIE1 MCU DMA1 */
		Self { phys: 0x7c000000, maps: 0xf0000, size: 0x10000 }, /* CONN_INFRA */
		Self { phys: 0x7c020000, maps: 0xd0000, size: 0x10000 }, /* CONN_INFRA, WFDMA */
		Self { phys: 0x7c060000, maps: 0xe0000, size: 0x10000 }, /* CONN_INFRA, conn_host_csr_top */
		Self { phys: 0x80020000, maps: 0xb0000, size: 0x10000 }, /* WF_TOP_MISC_OFF */
		Self { phys: 0x81020000, maps: 0xc0000, size: 0x10000 }, /* WF_TOP_MISC_ON */
		Self { phys: 0x820c0000, maps: 0x08000, size: 0x04000 }, /* WF_UMAC_TOP (PLE) */
		Self { phys: 0x820c8000, maps: 0x0c000, size: 0x02000 }, /* WF_UMAC_TOP (PSE) */
		Self { phys: 0x820cc000, maps: 0x0e000, size: 0x01000 }, /* WF_UMAC_TOP (PP) */
		Self { phys: 0x820cd000, maps: 0x0f000, size: 0x01000 }, /* WF_MDP_TOP */
		Self { phys: 0x74030000, maps: 0x10000, size: 0x10000 }, /* PCIE_MAC_IREG */
		Self { phys: 0x820ce000, maps: 0x21c00, size: 0x00200 }, /* WF_LMAC_TOP (WF_SEC) */
		Self { phys: 0x820cf000, maps: 0x22000, size: 0x01000 }, /* WF_LMAC_TOP (WF_PF) */
		Self { phys: 0x820e0000, maps: 0x20000, size: 0x00400 }, /* WF_LMAC_TOP BN0 (WF_CFG) */
		Self { phys: 0x820e1000, maps: 0x20400, size: 0x00200 }, /* WF_LMAC_TOP BN0 (WF_TRB) */
		Self { phys: 0x820e9000, maps: 0x23400, size: 0x00200 }, /* WF_LMAC_TOP BN0 (WF_WTBLOFF) */
		Self { phys: 0x820ea000, maps: 0x24000, size: 0x00200 }, /* WF_LMAC_TOP BN0 (WF_ETBF) */
		Self { phys: 0x820ec000, maps: 0x24600, size: 0x00200 }, /* WF_LMAC_TOP BN0 (WF_INT) */
		Self { phys: 0x820f0000, maps: 0xa0000, size: 0x00400 }, /* WF_LMAC_TOP BN1 (WF_CFG) */
		Self { phys: 0x820f1000, maps: 0xa0600, size: 0x00200 }, /* WF_LMAC_TOP BN1 (WF_TRB) */
		Self { phys: 0x820f2000, maps: 0xa0800, size: 0x00400 }, /* WF_LMAC_TOP BN1 (WF_AGG) */
		Self { phys: 0x820f3000, maps: 0xa0c00, size: 0x00400 }, /* WF_LMAC_TOP BN1 (WF_ARB) */
		Self { phys: 0x820f4000, maps: 0xa1000, size: 0x00400 }, /* WF_LMAC_TOP BN1 (WF_TMAC) */
		Self { phys: 0x820f5000, maps: 0xa1400, size: 0x00800 }, /* WF_LMAC_TOP BN1 (WF_RMAC) */
		Self { phys: 0x820f7000, maps: 0xa1e00, size: 0x00200 }, /* WF_LMAC_TOP BN1 (WF_DMA) */
		Self { phys: 0x820f9000, maps: 0xa3400, size: 0x00200 }, /* WF_LMAC_TOP BN1 (WF_WTBLOFF) */
		Self { phys: 0x820fa000, maps: 0xa4000, size: 0x00200 }, /* WF_LMAC_TOP BN1 (WF_ETBF) */
		Self { phys: 0x820fb000, maps: 0xa4200, size: 0x00400 }, /* WF_LMAC_TOP BN1 (WF_LPON) */
		Self { phys: 0x820fc000, maps: 0xa4600, size: 0x00200 }, /* WF_LMAC_TOP BN1 (WF_INT) */
		Self { phys: 0x820fd000, maps: 0xa4800, size: 0x00800 }, /* WF_LMAC_TOP BN1 (WF_MIB) */
	];

	pub(crate) fn get_offset(&self, address: u32) -> Option<u32> {
		if address < self.phys {
			return None;
		}

		let ofs = address - self.phys;
		if ofs > self.size {
			return None;
		}

		Some(self.maps + ofs)
	}
}
