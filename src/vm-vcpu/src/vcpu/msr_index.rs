// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the THIRD-PARTY file.

/*
 * automatically generated by rust-bindgen
 * From upstream linux msr-index.h at commit:
 * 806276b7f07a39a1cc3f38bb1ef5c573d4594a38
 */
#![cfg(target_arch = "x86_64")]

#![allow(non_upper_case_globals, unused, clippy::unreadable_literal)]

pub const MSR_EFER: ::std::os::raw::c_uint = 0xc0000080;
pub const MSR_STAR: ::std::os::raw::c_uint = 0xc0000081;
pub const MSR_LSTAR: ::std::os::raw::c_uint = 0xc0000082;
pub const MSR_CSTAR: ::std::os::raw::c_uint = 0xc0000083;
pub const MSR_SYSCALL_MASK: ::std::os::raw::c_uint = 0xc0000084;
pub const MSR_FS_BASE: ::std::os::raw::c_uint = 0xc0000100;
pub const MSR_GS_BASE: ::std::os::raw::c_uint = 0xc0000101;
pub const MSR_KERNEL_GS_BASE: ::std::os::raw::c_uint = 0xc0000102;
pub const MSR_TSC_AUX: ::std::os::raw::c_uint = 0xc0000103;
pub const _EFER_SCE: ::std::os::raw::c_uint = 0x00000000;
pub const _EFER_LME: ::std::os::raw::c_uint = 0x00000008;
pub const _EFER_LMA: ::std::os::raw::c_uint = 0x0000000a;
pub const _EFER_NX: ::std::os::raw::c_uint = 0x0000000b;
pub const _EFER_SVME: ::std::os::raw::c_uint = 0x0000000c;
pub const _EFER_LMSLE: ::std::os::raw::c_uint = 0x0000000d;
pub const _EFER_FFXSR: ::std::os::raw::c_uint = 0x0000000e;
pub const EFER_SCE: ::std::os::raw::c_uint = 0x00000001;
pub const EFER_LME: ::std::os::raw::c_uint = 0x00000100;
pub const EFER_LMA: ::std::os::raw::c_uint = 0x00000400;
pub const EFER_NX: ::std::os::raw::c_uint = 0x00000800;
pub const EFER_SVME: ::std::os::raw::c_uint = 0x00001000;
pub const EFER_LMSLE: ::std::os::raw::c_uint = 0x00002000;
pub const EFER_FFXSR: ::std::os::raw::c_uint = 0x00004000;
pub const MSR_PPIN_CTL: ::std::os::raw::c_uint = 0x0000004e;
pub const MSR_PPIN: ::std::os::raw::c_uint = 0x0000004f;
pub const MSR_IA32_PERFCTR0: ::std::os::raw::c_uint = 0x000000c1;
pub const MSR_IA32_PERFCTR1: ::std::os::raw::c_uint = 0x000000c2;
pub const MSR_FSB_FREQ: ::std::os::raw::c_uint = 0x000000cd;
pub const MSR_PLATFORM_INFO: ::std::os::raw::c_uint = 0x000000ce;
pub const MSR_PKG_CST_CONFIG_CONTROL: ::std::os::raw::c_uint = 0x000000e2;
pub const NHM_C3_AUTO_DEMOTE: ::std::os::raw::c_uint = 0x02000000;
pub const NHM_C1_AUTO_DEMOTE: ::std::os::raw::c_uint = 0x04000000;
pub const ATM_LNC_C6_AUTO_DEMOTE: ::std::os::raw::c_uint = 0x02000000;
pub const SNB_C1_AUTO_UNDEMOTE: ::std::os::raw::c_uint = 0x08000000;
pub const SNB_C3_AUTO_UNDEMOTE: ::std::os::raw::c_uint = 0x10000000;
pub const MSR_MTRRcap: ::std::os::raw::c_uint = 0x000000fe;
pub const MSR_IA32_BBL_CR_CTL: ::std::os::raw::c_uint = 0x00000119;
pub const MSR_IA32_BBL_CR_CTL3: ::std::os::raw::c_uint = 0x0000011e;
pub const MSR_IA32_SYSENTER_CS: ::std::os::raw::c_uint = 0x00000174;
pub const MSR_IA32_SYSENTER_ESP: ::std::os::raw::c_uint = 0x00000175;
pub const MSR_IA32_SYSENTER_EIP: ::std::os::raw::c_uint = 0x00000176;
pub const MSR_IA32_MCG_CAP: ::std::os::raw::c_uint = 0x00000179;
pub const MSR_IA32_MCG_STATUS: ::std::os::raw::c_uint = 0x0000017a;
pub const MSR_IA32_MCG_CTL: ::std::os::raw::c_uint = 0x0000017b;
pub const MSR_IA32_MCG_EXT_CTL: ::std::os::raw::c_uint = 0x000004d0;
pub const MSR_OFFCORE_RSP_0: ::std::os::raw::c_uint = 0x000001a6;
pub const MSR_OFFCORE_RSP_1: ::std::os::raw::c_uint = 0x000001a7;
pub const MSR_TURBO_RATIO_LIMIT: ::std::os::raw::c_uint = 0x000001ad;
pub const MSR_TURBO_RATIO_LIMIT1: ::std::os::raw::c_uint = 0x000001ae;
pub const MSR_TURBO_RATIO_LIMIT2: ::std::os::raw::c_uint = 0x000001af;
pub const MSR_LBR_SELECT: ::std::os::raw::c_uint = 0x000001c8;
pub const MSR_LBR_TOS: ::std::os::raw::c_uint = 0x000001c9;
pub const MSR_LBR_NHM_FROM: ::std::os::raw::c_uint = 0x00000680;
pub const MSR_LBR_NHM_TO: ::std::os::raw::c_uint = 0x000006c0;
pub const MSR_LBR_CORE_FROM: ::std::os::raw::c_uint = 0x00000040;
pub const MSR_LBR_CORE_TO: ::std::os::raw::c_uint = 0x00000060;
pub const MSR_LBR_INFO_0: ::std::os::raw::c_uint = 0x00000dc0;
pub const LBR_INFO_CYCLES: ::std::os::raw::c_uint = 0x0000ffff;
pub const MSR_IA32_PEBS_ENABLE: ::std::os::raw::c_uint = 0x000003f1;
pub const MSR_IA32_DS_AREA: ::std::os::raw::c_uint = 0x00000600;
pub const MSR_IA32_PERF_CAPABILITIES: ::std::os::raw::c_uint = 0x00000345;
pub const MSR_PEBS_LD_LAT_THRESHOLD: ::std::os::raw::c_uint = 0x000003f6;
pub const MSR_IA32_RTIT_CTL: ::std::os::raw::c_uint = 0x00000570;
pub const MSR_IA32_RTIT_STATUS: ::std::os::raw::c_uint = 0x00000571;
pub const MSR_IA32_RTIT_ADDR0_A: ::std::os::raw::c_uint = 0x00000580;
pub const MSR_IA32_RTIT_ADDR0_B: ::std::os::raw::c_uint = 0x00000581;
pub const MSR_IA32_RTIT_ADDR1_A: ::std::os::raw::c_uint = 0x00000582;
pub const MSR_IA32_RTIT_ADDR1_B: ::std::os::raw::c_uint = 0x00000583;
pub const MSR_IA32_RTIT_ADDR2_A: ::std::os::raw::c_uint = 0x00000584;
pub const MSR_IA32_RTIT_ADDR2_B: ::std::os::raw::c_uint = 0x00000585;
pub const MSR_IA32_RTIT_ADDR3_A: ::std::os::raw::c_uint = 0x00000586;
pub const MSR_IA32_RTIT_ADDR3_B: ::std::os::raw::c_uint = 0x00000587;
pub const MSR_IA32_RTIT_CR3_MATCH: ::std::os::raw::c_uint = 0x00000572;
pub const MSR_IA32_RTIT_OUTPUT_BASE: ::std::os::raw::c_uint = 0x00000560;
pub const MSR_IA32_RTIT_OUTPUT_MASK: ::std::os::raw::c_uint = 0x00000561;
pub const MSR_MTRRfix64K_00000: ::std::os::raw::c_uint = 0x00000250;
pub const MSR_MTRRfix16K_80000: ::std::os::raw::c_uint = 0x00000258;
pub const MSR_MTRRfix16K_A0000: ::std::os::raw::c_uint = 0x00000259;
pub const MSR_MTRRfix4K_C0000: ::std::os::raw::c_uint = 0x00000268;
pub const MSR_MTRRfix4K_C8000: ::std::os::raw::c_uint = 0x00000269;
pub const MSR_MTRRfix4K_D0000: ::std::os::raw::c_uint = 0x0000026a;
pub const MSR_MTRRfix4K_D8000: ::std::os::raw::c_uint = 0x0000026b;
pub const MSR_MTRRfix4K_E0000: ::std::os::raw::c_uint = 0x0000026c;
pub const MSR_MTRRfix4K_E8000: ::std::os::raw::c_uint = 0x0000026d;
pub const MSR_MTRRfix4K_F0000: ::std::os::raw::c_uint = 0x0000026e;
pub const MSR_MTRRfix4K_F8000: ::std::os::raw::c_uint = 0x0000026f;
pub const MSR_MTRRdefType: ::std::os::raw::c_uint = 0x000002ff;
pub const MSR_IA32_CR_PAT: ::std::os::raw::c_uint = 0x00000277;
pub const MSR_IA32_DEBUGCTLMSR: ::std::os::raw::c_uint = 0x000001d9;
pub const MSR_IA32_LASTBRANCHFROMIP: ::std::os::raw::c_uint = 0x000001db;
pub const MSR_IA32_LASTBRANCHTOIP: ::std::os::raw::c_uint = 0x000001dc;
pub const MSR_IA32_LASTINTFROMIP: ::std::os::raw::c_uint = 0x000001dd;
pub const MSR_IA32_LASTINTTOIP: ::std::os::raw::c_uint = 0x000001de;
pub const DEBUGCTLMSR_LBR: ::std::os::raw::c_uint = 0x00000001;
pub const DEBUGCTLMSR_BTF: ::std::os::raw::c_uint = 0x00000002;
pub const DEBUGCTLMSR_TR: ::std::os::raw::c_uint = 0x00000040;
pub const DEBUGCTLMSR_BTS: ::std::os::raw::c_uint = 0x00000080;
pub const DEBUGCTLMSR_BTINT: ::std::os::raw::c_uint = 0x00000100;
pub const DEBUGCTLMSR_BTS_OFF_OS: ::std::os::raw::c_uint = 0x00000200;
pub const DEBUGCTLMSR_BTS_OFF_USR: ::std::os::raw::c_uint = 0x00000400;
pub const DEBUGCTLMSR_FREEZE_LBRS_ON_PMI: ::std::os::raw::c_uint = 0x00000800;
pub const MSR_PEBS_FRONTEND: ::std::os::raw::c_uint = 0x000003f7;
pub const MSR_IA32_POWER_CTL: ::std::os::raw::c_uint = 0x000001fc;
pub const MSR_IA32_MC0_CTL: ::std::os::raw::c_uint = 0x00000400;
pub const MSR_IA32_MC0_STATUS: ::std::os::raw::c_uint = 0x00000401;
pub const MSR_IA32_MC0_ADDR: ::std::os::raw::c_uint = 0x00000402;
pub const MSR_IA32_MC0_MISC: ::std::os::raw::c_uint = 0x00000403;
pub const MSR_PKG_C3_RESIDENCY: ::std::os::raw::c_uint = 0x000003f8;
pub const MSR_PKG_C6_RESIDENCY: ::std::os::raw::c_uint = 0x000003f9;
pub const MSR_ATOM_PKG_C6_RESIDENCY: ::std::os::raw::c_uint = 0x000003fa;
pub const MSR_PKG_C7_RESIDENCY: ::std::os::raw::c_uint = 0x000003fa;
pub const MSR_CORE_C3_RESIDENCY: ::std::os::raw::c_uint = 0x000003fc;
pub const MSR_CORE_C6_RESIDENCY: ::std::os::raw::c_uint = 0x000003fd;
pub const MSR_CORE_C7_RESIDENCY: ::std::os::raw::c_uint = 0x000003fe;
pub const MSR_KNL_CORE_C6_RESIDENCY: ::std::os::raw::c_uint = 0x000003ff;
pub const MSR_PKG_C2_RESIDENCY: ::std::os::raw::c_uint = 0x0000060d;
pub const MSR_PKG_C8_RESIDENCY: ::std::os::raw::c_uint = 0x00000630;
pub const MSR_PKG_C9_RESIDENCY: ::std::os::raw::c_uint = 0x00000631;
pub const MSR_PKG_C10_RESIDENCY: ::std::os::raw::c_uint = 0x00000632;
pub const MSR_PKGC3_IRTL: ::std::os::raw::c_uint = 0x0000060a;
pub const MSR_PKGC6_IRTL: ::std::os::raw::c_uint = 0x0000060b;
pub const MSR_PKGC7_IRTL: ::std::os::raw::c_uint = 0x0000060c;
pub const MSR_PKGC8_IRTL: ::std::os::raw::c_uint = 0x00000633;
pub const MSR_PKGC9_IRTL: ::std::os::raw::c_uint = 0x00000634;
pub const MSR_PKGC10_IRTL: ::std::os::raw::c_uint = 0x00000635;
pub const MSR_RAPL_POWER_UNIT: ::std::os::raw::c_uint = 0x00000606;
pub const MSR_PKG_POWER_LIMIT: ::std::os::raw::c_uint = 0x00000610;
pub const MSR_PKG_ENERGY_STATUS: ::std::os::raw::c_uint = 0x00000611;
pub const MSR_PKG_PERF_STATUS: ::std::os::raw::c_uint = 0x00000613;
pub const MSR_PKG_POWER_INFO: ::std::os::raw::c_uint = 0x00000614;
pub const MSR_DRAM_POWER_LIMIT: ::std::os::raw::c_uint = 0x00000618;
pub const MSR_DRAM_ENERGY_STATUS: ::std::os::raw::c_uint = 0x00000619;
pub const MSR_DRAM_PERF_STATUS: ::std::os::raw::c_uint = 0x0000061b;
pub const MSR_DRAM_POWER_INFO: ::std::os::raw::c_uint = 0x0000061c;
pub const MSR_PP0_POWER_LIMIT: ::std::os::raw::c_uint = 0x00000638;
pub const MSR_PP0_ENERGY_STATUS: ::std::os::raw::c_uint = 0x00000639;
pub const MSR_PP0_POLICY: ::std::os::raw::c_uint = 0x0000063a;
pub const MSR_PP0_PERF_STATUS: ::std::os::raw::c_uint = 0x0000063b;
pub const MSR_PP1_POWER_LIMIT: ::std::os::raw::c_uint = 0x00000640;
pub const MSR_PP1_ENERGY_STATUS: ::std::os::raw::c_uint = 0x00000641;
pub const MSR_PP1_POLICY: ::std::os::raw::c_uint = 0x00000642;
pub const MSR_CONFIG_TDP_NOMINAL: ::std::os::raw::c_uint = 0x00000648;
pub const MSR_CONFIG_TDP_LEVEL_1: ::std::os::raw::c_uint = 0x00000649;
pub const MSR_CONFIG_TDP_LEVEL_2: ::std::os::raw::c_uint = 0x0000064a;
pub const MSR_CONFIG_TDP_CONTROL: ::std::os::raw::c_uint = 0x0000064b;
pub const MSR_TURBO_ACTIVATION_RATIO: ::std::os::raw::c_uint = 0x0000064c;
pub const MSR_PLATFORM_ENERGY_STATUS: ::std::os::raw::c_uint = 0x0000064d;
pub const MSR_PKG_WEIGHTED_CORE_C0_RES: ::std::os::raw::c_uint = 0x00000658;
pub const MSR_PKG_ANY_CORE_C0_RES: ::std::os::raw::c_uint = 0x00000659;
pub const MSR_PKG_ANY_GFXE_C0_RES: ::std::os::raw::c_uint = 0x0000065a;
pub const MSR_PKG_BOTH_CORE_GFXE_C0_RES: ::std::os::raw::c_uint = 0x0000065b;
pub const MSR_CORE_C1_RES: ::std::os::raw::c_uint = 0x00000660;
pub const MSR_MODULE_C6_RES_MS: ::std::os::raw::c_uint = 0x00000664;
pub const MSR_CC6_DEMOTION_POLICY_CONFIG: ::std::os::raw::c_uint = 0x00000668;
pub const MSR_MC6_DEMOTION_POLICY_CONFIG: ::std::os::raw::c_uint = 0x00000669;
pub const MSR_ATOM_CORE_RATIOS: ::std::os::raw::c_uint = 0x0000066a;
pub const MSR_ATOM_CORE_VIDS: ::std::os::raw::c_uint = 0x0000066b;
pub const MSR_ATOM_CORE_TURBO_RATIOS: ::std::os::raw::c_uint = 0x0000066c;
pub const MSR_ATOM_CORE_TURBO_VIDS: ::std::os::raw::c_uint = 0x0000066d;
pub const MSR_CORE_PERF_LIMIT_REASONS: ::std::os::raw::c_uint = 0x00000690;
pub const MSR_GFX_PERF_LIMIT_REASONS: ::std::os::raw::c_uint = 0x000006b0;
pub const MSR_RING_PERF_LIMIT_REASONS: ::std::os::raw::c_uint = 0x000006b1;
pub const MSR_PPERF: ::std::os::raw::c_uint = 0x0000064e;
pub const MSR_PERF_LIMIT_REASONS: ::std::os::raw::c_uint = 0x0000064f;
pub const MSR_PM_ENABLE: ::std::os::raw::c_uint = 0x00000770;
pub const MSR_HWP_CAPABILITIES: ::std::os::raw::c_uint = 0x00000771;
pub const MSR_HWP_REQUEST_PKG: ::std::os::raw::c_uint = 0x00000772;
pub const MSR_HWP_INTERRUPT: ::std::os::raw::c_uint = 0x00000773;
pub const MSR_HWP_REQUEST: ::std::os::raw::c_uint = 0x00000774;
pub const MSR_HWP_STATUS: ::std::os::raw::c_uint = 0x00000777;
pub const HWP_BASE_BIT: ::std::os::raw::c_uint = 0x00000080;
pub const HWP_NOTIFICATIONS_BIT: ::std::os::raw::c_uint = 0x00000100;
pub const HWP_ACTIVITY_WINDOW_BIT: ::std::os::raw::c_uint = 0x00000200;
pub const HWP_ENERGY_PERF_PREFERENCE_BIT: ::std::os::raw::c_uint = 0x00000400;
pub const HWP_PACKAGE_LEVEL_REQUEST_BIT: ::std::os::raw::c_uint = 0x00000800;
pub const MSR_AMD64_MC0_MASK: ::std::os::raw::c_uint = 0xc0010044;
pub const MSR_IA32_MC0_CTL2: ::std::os::raw::c_uint = 0x00000280;
pub const MSR_P6_PERFCTR0: ::std::os::raw::c_uint = 0x000000c1;
pub const MSR_P6_PERFCTR1: ::std::os::raw::c_uint = 0x000000c2;
pub const MSR_P6_EVNTSEL0: ::std::os::raw::c_uint = 0x00000186;
pub const MSR_P6_EVNTSEL1: ::std::os::raw::c_uint = 0x00000187;
pub const MSR_KNC_PERFCTR0: ::std::os::raw::c_uint = 0x00000020;
pub const MSR_KNC_PERFCTR1: ::std::os::raw::c_uint = 0x00000021;
pub const MSR_KNC_EVNTSEL0: ::std::os::raw::c_uint = 0x00000028;
pub const MSR_KNC_EVNTSEL1: ::std::os::raw::c_uint = 0x00000029;
pub const MSR_IA32_PMC0: ::std::os::raw::c_uint = 0x000004c1;
pub const MSR_AMD64_PATCH_LEVEL: ::std::os::raw::c_uint = 0x0000008b;
pub const MSR_AMD64_TSC_RATIO: ::std::os::raw::c_uint = 0xc0000104;
pub const MSR_AMD64_NB_CFG: ::std::os::raw::c_uint = 0xc001001f;
pub const MSR_AMD64_PATCH_LOADER: ::std::os::raw::c_uint = 0xc0010020;
pub const MSR_AMD64_OSVW_ID_LENGTH: ::std::os::raw::c_uint = 0xc0010140;
pub const MSR_AMD64_OSVW_STATUS: ::std::os::raw::c_uint = 0xc0010141;
pub const MSR_AMD64_LS_CFG: ::std::os::raw::c_uint = 0xc0011020;
pub const MSR_AMD64_DC_CFG: ::std::os::raw::c_uint = 0xc0011022;
pub const MSR_AMD64_BU_CFG2: ::std::os::raw::c_uint = 0xc001102a;
pub const MSR_AMD64_IBSFETCHCTL: ::std::os::raw::c_uint = 0xc0011030;
pub const MSR_AMD64_IBSFETCHLINAD: ::std::os::raw::c_uint = 0xc0011031;
pub const MSR_AMD64_IBSFETCHPHYSAD: ::std::os::raw::c_uint = 0xc0011032;
pub const MSR_AMD64_IBSFETCH_REG_COUNT: ::std::os::raw::c_uint = 0x00000003;
pub const MSR_AMD64_IBSFETCH_REG_MASK: ::std::os::raw::c_uint = 0x00000007;
pub const MSR_AMD64_IBSOPCTL: ::std::os::raw::c_uint = 0xc0011033;
pub const MSR_AMD64_IBSOPRIP: ::std::os::raw::c_uint = 0xc0011034;
pub const MSR_AMD64_IBSOPDATA: ::std::os::raw::c_uint = 0xc0011035;
pub const MSR_AMD64_IBSOPDATA2: ::std::os::raw::c_uint = 0xc0011036;
pub const MSR_AMD64_IBSOPDATA3: ::std::os::raw::c_uint = 0xc0011037;
pub const MSR_AMD64_IBSDCLINAD: ::std::os::raw::c_uint = 0xc0011038;
pub const MSR_AMD64_IBSDCPHYSAD: ::std::os::raw::c_uint = 0xc0011039;
pub const MSR_AMD64_IBSOP_REG_COUNT: ::std::os::raw::c_uint = 0x00000007;
pub const MSR_AMD64_IBSOP_REG_MASK: ::std::os::raw::c_uint = 0x0000007f;
pub const MSR_AMD64_IBSCTL: ::std::os::raw::c_uint = 0xc001103a;
pub const MSR_AMD64_IBSBRTARGET: ::std::os::raw::c_uint = 0xc001103b;
pub const MSR_AMD64_IBSOPDATA4: ::std::os::raw::c_uint = 0xc001103d;
pub const MSR_AMD64_IBS_REG_COUNT_MAX: ::std::os::raw::c_uint = 0x00000008;
pub const MSR_F17H_IRPERF: ::std::os::raw::c_uint = 0xc00000e9;
pub const MSR_F16H_L2I_PERF_CTL: ::std::os::raw::c_uint = 0xc0010230;
pub const MSR_F16H_L2I_PERF_CTR: ::std::os::raw::c_uint = 0xc0010231;
pub const MSR_F16H_DR1_ADDR_MASK: ::std::os::raw::c_uint = 0xc0011019;
pub const MSR_F16H_DR2_ADDR_MASK: ::std::os::raw::c_uint = 0xc001101a;
pub const MSR_F16H_DR3_ADDR_MASK: ::std::os::raw::c_uint = 0xc001101b;
pub const MSR_F16H_DR0_ADDR_MASK: ::std::os::raw::c_uint = 0xc0011027;
pub const MSR_F15H_PERF_CTL: ::std::os::raw::c_uint = 0xc0010200;
pub const MSR_F15H_PERF_CTR: ::std::os::raw::c_uint = 0xc0010201;
pub const MSR_F15H_NB_PERF_CTL: ::std::os::raw::c_uint = 0xc0010240;
pub const MSR_F15H_NB_PERF_CTR: ::std::os::raw::c_uint = 0xc0010241;
pub const MSR_F15H_PTSC: ::std::os::raw::c_uint = 0xc0010280;
pub const MSR_F15H_IC_CFG: ::std::os::raw::c_uint = 0xc0011021;
pub const MSR_FAM10H_MMIO_CONF_BASE: ::std::os::raw::c_uint = 0xc0010058;
pub const FAM10H_MMIO_CONF_ENABLE: ::std::os::raw::c_uint = 0x00000001;
pub const FAM10H_MMIO_CONF_BUSRANGE_MASK: ::std::os::raw::c_uint = 0x0000000f;
pub const FAM10H_MMIO_CONF_BUSRANGE_SHIFT: ::std::os::raw::c_uint = 0x00000002;
pub const FAM10H_MMIO_CONF_BASE_MASK: ::std::os::raw::c_uint = 0x0fffffff;
pub const FAM10H_MMIO_CONF_BASE_SHIFT: ::std::os::raw::c_uint = 0x00000014;
pub const MSR_FAM10H_NODE_ID: ::std::os::raw::c_uint = 0xc001100c;
pub const MSR_K8_TOP_MEM1: ::std::os::raw::c_uint = 0xc001001a;
pub const MSR_K8_TOP_MEM2: ::std::os::raw::c_uint = 0xc001001d;
pub const MSR_K8_SYSCFG: ::std::os::raw::c_uint = 0xc0010010;
pub const MSR_K8_INT_PENDING_MSG: ::std::os::raw::c_uint = 0xc0010055;
pub const K8_INTP_C1E_ACTIVE_MASK: ::std::os::raw::c_uint = 0x18000000;
pub const MSR_K8_TSEG_ADDR: ::std::os::raw::c_uint = 0xc0010112;
pub const MSR_K8_TSEG_MASK: ::std::os::raw::c_uint = 0xc0010113;
pub const K8_MTRRFIXRANGE_DRAM_ENABLE: ::std::os::raw::c_uint = 0x00040000;
pub const K8_MTRRFIXRANGE_DRAM_MODIFY: ::std::os::raw::c_uint = 0x00080000;
pub const K8_MTRR_RDMEM_WRMEM_MASK: ::std::os::raw::c_uint = 0x18181818;
pub const MSR_K7_EVNTSEL0: ::std::os::raw::c_uint = 0xc0010000;
pub const MSR_K7_PERFCTR0: ::std::os::raw::c_uint = 0xc0010004;
pub const MSR_K7_EVNTSEL1: ::std::os::raw::c_uint = 0xc0010001;
pub const MSR_K7_PERFCTR1: ::std::os::raw::c_uint = 0xc0010005;
pub const MSR_K7_EVNTSEL2: ::std::os::raw::c_uint = 0xc0010002;
pub const MSR_K7_PERFCTR2: ::std::os::raw::c_uint = 0xc0010006;
pub const MSR_K7_EVNTSEL3: ::std::os::raw::c_uint = 0xc0010003;
pub const MSR_K7_PERFCTR3: ::std::os::raw::c_uint = 0xc0010007;
pub const MSR_K7_CLK_CTL: ::std::os::raw::c_uint = 0xc001001b;
pub const MSR_K7_HWCR: ::std::os::raw::c_uint = 0xc0010015;
pub const MSR_K7_FID_VID_CTL: ::std::os::raw::c_uint = 0xc0010041;
pub const MSR_K7_FID_VID_STATUS: ::std::os::raw::c_uint = 0xc0010042;
pub const MSR_K6_WHCR: ::std::os::raw::c_uint = 0xc0000082;
pub const MSR_K6_UWCCR: ::std::os::raw::c_uint = 0xc0000085;
pub const MSR_K6_EPMR: ::std::os::raw::c_uint = 0xc0000086;
pub const MSR_K6_PSOR: ::std::os::raw::c_uint = 0xc0000087;
pub const MSR_K6_PFIR: ::std::os::raw::c_uint = 0xc0000088;
pub const MSR_IDT_FCR1: ::std::os::raw::c_uint = 0x00000107;
pub const MSR_IDT_FCR2: ::std::os::raw::c_uint = 0x00000108;
pub const MSR_IDT_FCR3: ::std::os::raw::c_uint = 0x00000109;
pub const MSR_IDT_FCR4: ::std::os::raw::c_uint = 0x0000010a;
pub const MSR_IDT_MCR0: ::std::os::raw::c_uint = 0x00000110;
pub const MSR_IDT_MCR1: ::std::os::raw::c_uint = 0x00000111;
pub const MSR_IDT_MCR2: ::std::os::raw::c_uint = 0x00000112;
pub const MSR_IDT_MCR3: ::std::os::raw::c_uint = 0x00000113;
pub const MSR_IDT_MCR4: ::std::os::raw::c_uint = 0x00000114;
pub const MSR_IDT_MCR5: ::std::os::raw::c_uint = 0x00000115;
pub const MSR_IDT_MCR6: ::std::os::raw::c_uint = 0x00000116;
pub const MSR_IDT_MCR7: ::std::os::raw::c_uint = 0x00000117;
pub const MSR_IDT_MCR_CTRL: ::std::os::raw::c_uint = 0x00000120;
pub const MSR_VIA_FCR: ::std::os::raw::c_uint = 0x00001107;
pub const MSR_VIA_LONGHAUL: ::std::os::raw::c_uint = 0x0000110a;
pub const MSR_VIA_RNG: ::std::os::raw::c_uint = 0x0000110b;
pub const MSR_VIA_BCR2: ::std::os::raw::c_uint = 0x00001147;
pub const MSR_TMTA_LONGRUN_CTRL: ::std::os::raw::c_uint = 0x80868010;
pub const MSR_TMTA_LONGRUN_FLAGS: ::std::os::raw::c_uint = 0x80868011;
pub const MSR_TMTA_LRTI_READOUT: ::std::os::raw::c_uint = 0x80868018;
pub const MSR_TMTA_LRTI_VOLT_MHZ: ::std::os::raw::c_uint = 0x8086801a;
pub const MSR_IA32_P5_MC_ADDR: ::std::os::raw::c_uint = 0x00000000;
pub const MSR_IA32_P5_MC_TYPE: ::std::os::raw::c_uint = 0x00000001;
pub const MSR_IA32_TSC: ::std::os::raw::c_uint = 0x00000010;
pub const MSR_IA32_PLATFORM_ID: ::std::os::raw::c_uint = 0x00000017;
pub const MSR_IA32_EBL_CR_POWERON: ::std::os::raw::c_uint = 0x0000002a;
pub const MSR_EBC_FREQUENCY_ID: ::std::os::raw::c_uint = 0x0000002c;
pub const MSR_SMI_COUNT: ::std::os::raw::c_uint = 0x00000034;
pub const MSR_IA32_FEATURE_CONTROL: ::std::os::raw::c_uint = 0x0000003a;
pub const MSR_IA32_TSC_ADJUST: ::std::os::raw::c_uint = 0x0000003b;
pub const MSR_IA32_BNDCFGS: ::std::os::raw::c_uint = 0x00000d90;
pub const MSR_IA32_XSS: ::std::os::raw::c_uint = 0x00000da0;
pub const FEATURE_CONTROL_LOCKED: ::std::os::raw::c_uint = 0x00000001;
pub const FEATURE_CONTROL_VMXON_ENABLED_INSIDE_SMX: ::std::os::raw::c_uint = 0x00000002;
pub const FEATURE_CONTROL_VMXON_ENABLED_OUTSIDE_SMX: ::std::os::raw::c_uint = 0x00000004;
pub const FEATURE_CONTROL_LMCE: ::std::os::raw::c_uint = 0x00100000;
pub const MSR_IA32_APICBASE: ::std::os::raw::c_uint = 0x0000001b;
pub const MSR_IA32_APICBASE_BSP: ::std::os::raw::c_uint = 0x00000100;
pub const MSR_IA32_APICBASE_ENABLE: ::std::os::raw::c_uint = 0x00000800;
pub const MSR_IA32_APICBASE_BASE: ::std::os::raw::c_uint = 0xfffff000;
pub const MSR_IA32_TSCDEADLINE: ::std::os::raw::c_uint = 0x000006e0;
pub const MSR_IA32_UCODE_WRITE: ::std::os::raw::c_uint = 0x00000079;
pub const MSR_IA32_UCODE_REV: ::std::os::raw::c_uint = 0x0000008b;
pub const MSR_IA32_SMM_MONITOR_CTL: ::std::os::raw::c_uint = 0x0000009b;
pub const MSR_IA32_SMBASE: ::std::os::raw::c_uint = 0x0000009e;
pub const MSR_IA32_PERF_STATUS: ::std::os::raw::c_uint = 0x00000198;
pub const MSR_IA32_PERF_CTL: ::std::os::raw::c_uint = 0x00000199;
pub const INTEL_PERF_CTL_MASK: ::std::os::raw::c_uint = 0x0000ffff;
pub const MSR_AMD_PSTATE_DEF_BASE: ::std::os::raw::c_uint = 0xc0010064;
pub const MSR_AMD_PERF_STATUS: ::std::os::raw::c_uint = 0xc0010063;
pub const MSR_AMD_PERF_CTL: ::std::os::raw::c_uint = 0xc0010062;
pub const MSR_IA32_MPERF: ::std::os::raw::c_uint = 0x000000e7;
pub const MSR_IA32_APERF: ::std::os::raw::c_uint = 0x000000e8;
pub const MSR_IA32_THERM_CONTROL: ::std::os::raw::c_uint = 0x0000019a;
pub const MSR_IA32_THERM_INTERRUPT: ::std::os::raw::c_uint = 0x0000019b;
pub const THERM_INT_HIGH_ENABLE: ::std::os::raw::c_uint = 0x00000001;
pub const THERM_INT_LOW_ENABLE: ::std::os::raw::c_uint = 0x00000002;
pub const THERM_INT_PLN_ENABLE: ::std::os::raw::c_uint = 0x01000000;
pub const MSR_IA32_THERM_STATUS: ::std::os::raw::c_uint = 0x0000019c;
pub const THERM_STATUS_PROCHOT: ::std::os::raw::c_uint = 0x00000001;
pub const THERM_STATUS_POWER_LIMIT: ::std::os::raw::c_uint = 0x00000400;
pub const MSR_THERM2_CTL: ::std::os::raw::c_uint = 0x0000019d;
pub const MSR_THERM2_CTL_TM_SELECT: ::std::os::raw::c_uint = 0x00010000;
pub const MSR_IA32_MISC_ENABLE: ::std::os::raw::c_uint = 0x000001a0;
pub const MSR_IA32_TEMPERATURE_TARGET: ::std::os::raw::c_uint = 0x000001a2;
pub const MSR_MISC_FEATURE_CONTROL: ::std::os::raw::c_uint = 0x000001a4;
pub const MSR_MISC_PWR_MGMT: ::std::os::raw::c_uint = 0x000001aa;
pub const MSR_IA32_ENERGY_PERF_BIAS: ::std::os::raw::c_uint = 0x000001b0;
pub const ENERGY_PERF_BIAS_PERFORMANCE: ::std::os::raw::c_uint = 0x00000000;
pub const ENERGY_PERF_BIAS_NORMAL: ::std::os::raw::c_uint = 0x00000006;
pub const ENERGY_PERF_BIAS_POWERSAVE: ::std::os::raw::c_uint = 0x0000000f;
pub const MSR_IA32_PACKAGE_THERM_STATUS: ::std::os::raw::c_uint = 0x000001b1;
pub const PACKAGE_THERM_STATUS_PROCHOT: ::std::os::raw::c_uint = 0x00000001;
pub const PACKAGE_THERM_STATUS_POWER_LIMIT: ::std::os::raw::c_uint = 0x00000400;
pub const MSR_IA32_PACKAGE_THERM_INTERRUPT: ::std::os::raw::c_uint = 0x000001b2;
pub const PACKAGE_THERM_INT_HIGH_ENABLE: ::std::os::raw::c_uint = 0x00000001;
pub const PACKAGE_THERM_INT_LOW_ENABLE: ::std::os::raw::c_uint = 0x00000002;
pub const PACKAGE_THERM_INT_PLN_ENABLE: ::std::os::raw::c_uint = 0x01000000;
pub const THERM_INT_THRESHOLD0_ENABLE: ::std::os::raw::c_uint = 0x00008000;
pub const THERM_SHIFT_THRESHOLD0: ::std::os::raw::c_uint = 0x00000008;
pub const THERM_MASK_THRESHOLD0: ::std::os::raw::c_uint = 0x00007f00;
pub const THERM_INT_THRESHOLD1_ENABLE: ::std::os::raw::c_uint = 0x00800000;
pub const THERM_SHIFT_THRESHOLD1: ::std::os::raw::c_uint = 0x00000010;
pub const THERM_MASK_THRESHOLD1: ::std::os::raw::c_uint = 0x007f0000;
pub const THERM_STATUS_THRESHOLD0: ::std::os::raw::c_uint = 0x00000040;
pub const THERM_LOG_THRESHOLD0: ::std::os::raw::c_uint = 0x00000080;
pub const THERM_STATUS_THRESHOLD1: ::std::os::raw::c_uint = 0x00000100;
pub const THERM_LOG_THRESHOLD1: ::std::os::raw::c_uint = 0x00000200;
pub const MSR_IA32_MISC_ENABLE_FAST_STRING_BIT: ::std::os::raw::c_uint = 0x00000000;
pub const MSR_IA32_MISC_ENABLE_FAST_STRING: ::std::os::raw::c_uint = 0x00000001;
pub const MSR_IA32_MISC_ENABLE_TCC_BIT: ::std::os::raw::c_uint = 0x00000001;
pub const MSR_IA32_MISC_ENABLE_TCC: ::std::os::raw::c_uint = 0x00000002;
pub const MSR_IA32_MISC_ENABLE_EMON_BIT: ::std::os::raw::c_uint = 0x00000007;
pub const MSR_IA32_MISC_ENABLE_EMON: ::std::os::raw::c_uint = 0x00000080;
pub const MSR_IA32_MISC_ENABLE_BTS_UNAVAIL_BIT: ::std::os::raw::c_uint = 0x0000000b;
pub const MSR_IA32_MISC_ENABLE_BTS_UNAVAIL: ::std::os::raw::c_uint = 0x00000800;
pub const MSR_IA32_MISC_ENABLE_PEBS_UNAVAIL_BIT: ::std::os::raw::c_uint = 0x0000000c;
pub const MSR_IA32_MISC_ENABLE_PEBS_UNAVAIL: ::std::os::raw::c_uint = 0x00001000;
pub const MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP_BIT: ::std::os::raw::c_uint = 0x00000010;
pub const MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP: ::std::os::raw::c_uint = 0x00010000;
pub const MSR_IA32_MISC_ENABLE_MWAIT_BIT: ::std::os::raw::c_uint = 0x00000012;
pub const MSR_IA32_MISC_ENABLE_MWAIT: ::std::os::raw::c_uint = 0x00040000;
pub const MSR_IA32_MISC_ENABLE_LIMIT_CPUID_BIT: ::std::os::raw::c_uint = 0x00000016;
pub const MSR_IA32_MISC_ENABLE_LIMIT_CPUID: ::std::os::raw::c_uint = 0x00400000;
pub const MSR_IA32_MISC_ENABLE_XTPR_DISABLE_BIT: ::std::os::raw::c_uint = 0x00000017;
pub const MSR_IA32_MISC_ENABLE_XTPR_DISABLE: ::std::os::raw::c_uint = 0x00800000;
pub const MSR_IA32_MISC_ENABLE_XD_DISABLE_BIT: ::std::os::raw::c_uint = 0x00000022;
pub const MSR_IA32_MISC_ENABLE_XD_DISABLE: ::std::os::raw::c_ulonglong = 0x400000000;
pub const MSR_IA32_MISC_ENABLE_X87_COMPAT_BIT: ::std::os::raw::c_uint = 0x00000002;
pub const MSR_IA32_MISC_ENABLE_X87_COMPAT: ::std::os::raw::c_uint = 0x00000004;
pub const MSR_IA32_MISC_ENABLE_TM1_BIT: ::std::os::raw::c_uint = 0x00000003;
pub const MSR_IA32_MISC_ENABLE_TM1: ::std::os::raw::c_uint = 0x00000008;
pub const MSR_IA32_MISC_ENABLE_SPLIT_LOCK_DISABLE_BIT: ::std::os::raw::c_uint = 0x00000004;
pub const MSR_IA32_MISC_ENABLE_SPLIT_LOCK_DISABLE: ::std::os::raw::c_uint = 0x00000010;
pub const MSR_IA32_MISC_ENABLE_L3CACHE_DISABLE_BIT: ::std::os::raw::c_uint = 0x00000006;
pub const MSR_IA32_MISC_ENABLE_L3CACHE_DISABLE: ::std::os::raw::c_uint = 0x00000040;
pub const MSR_IA32_MISC_ENABLE_SUPPRESS_LOCK_BIT: ::std::os::raw::c_uint = 0x00000008;
pub const MSR_IA32_MISC_ENABLE_SUPPRESS_LOCK: ::std::os::raw::c_uint = 0x00000100;
pub const MSR_IA32_MISC_ENABLE_PREFETCH_DISABLE_BIT: ::std::os::raw::c_uint = 0x00000009;
pub const MSR_IA32_MISC_ENABLE_PREFETCH_DISABLE: ::std::os::raw::c_uint = 0x00000200;
pub const MSR_IA32_MISC_ENABLE_FERR_BIT: ::std::os::raw::c_uint = 0x0000000a;
pub const MSR_IA32_MISC_ENABLE_FERR: ::std::os::raw::c_uint = 0x00000400;
pub const MSR_IA32_MISC_ENABLE_FERR_MULTIPLEX_BIT: ::std::os::raw::c_uint = 0x0000000a;
pub const MSR_IA32_MISC_ENABLE_FERR_MULTIPLEX: ::std::os::raw::c_uint = 0x00000400;
pub const MSR_IA32_MISC_ENABLE_TM2_BIT: ::std::os::raw::c_uint = 0x0000000d;
pub const MSR_IA32_MISC_ENABLE_TM2: ::std::os::raw::c_uint = 0x00002000;
pub const MSR_IA32_MISC_ENABLE_ADJ_PREF_DISABLE_BIT: ::std::os::raw::c_uint = 0x00000013;
pub const MSR_IA32_MISC_ENABLE_ADJ_PREF_DISABLE: ::std::os::raw::c_uint = 0x00080000;
pub const MSR_IA32_MISC_ENABLE_SPEEDSTEP_LOCK_BIT: ::std::os::raw::c_uint = 0x00000014;
pub const MSR_IA32_MISC_ENABLE_SPEEDSTEP_LOCK: ::std::os::raw::c_uint = 0x00100000;
pub const MSR_IA32_MISC_ENABLE_L1D_CONTEXT_BIT: ::std::os::raw::c_uint = 0x00000018;
pub const MSR_IA32_MISC_ENABLE_L1D_CONTEXT: ::std::os::raw::c_uint = 0x01000000;
pub const MSR_IA32_MISC_ENABLE_DCU_PREF_DISABLE_BIT: ::std::os::raw::c_uint = 0x00000025;
pub const MSR_IA32_MISC_ENABLE_DCU_PREF_DISABLE: ::std::os::raw::c_ulonglong = 0x2000000000;
pub const MSR_IA32_MISC_ENABLE_TURBO_DISABLE_BIT: ::std::os::raw::c_uint = 0x00000026;
pub const MSR_IA32_MISC_ENABLE_TURBO_DISABLE: ::std::os::raw::c_ulonglong = 0x4000000000;
pub const MSR_IA32_MISC_ENABLE_IP_PREF_DISABLE_BIT: ::std::os::raw::c_uint = 0x00000027;
pub const MSR_IA32_MISC_ENABLE_IP_PREF_DISABLE: ::std::os::raw::c_ulonglong = 0x8000000000;
pub const MSR_MISC_FEATURE_ENABLES: ::std::os::raw::c_uint = 0x00000140;
pub const MSR_MISC_FEATURE_ENABLES_RING3MWAIT_BIT: ::std::os::raw::c_uint = 0x00000001;
pub const MSR_IA32_TSC_DEADLINE: ::std::os::raw::c_uint = 0x000006e0;
pub const MSR_IA32_MCG_EAX: ::std::os::raw::c_uint = 0x00000180;
pub const MSR_IA32_MCG_EBX: ::std::os::raw::c_uint = 0x00000181;
pub const MSR_IA32_MCG_ECX: ::std::os::raw::c_uint = 0x00000182;
pub const MSR_IA32_MCG_EDX: ::std::os::raw::c_uint = 0x00000183;
pub const MSR_IA32_MCG_ESI: ::std::os::raw::c_uint = 0x00000184;
pub const MSR_IA32_MCG_EDI: ::std::os::raw::c_uint = 0x00000185;
pub const MSR_IA32_MCG_EBP: ::std::os::raw::c_uint = 0x00000186;
pub const MSR_IA32_MCG_ESP: ::std::os::raw::c_uint = 0x00000187;
pub const MSR_IA32_MCG_EFLAGS: ::std::os::raw::c_uint = 0x00000188;
pub const MSR_IA32_MCG_EIP: ::std::os::raw::c_uint = 0x00000189;
pub const MSR_IA32_MCG_RESERVED: ::std::os::raw::c_uint = 0x0000018a;
pub const MSR_P4_BPU_PERFCTR0: ::std::os::raw::c_uint = 0x00000300;
pub const MSR_P4_BPU_PERFCTR1: ::std::os::raw::c_uint = 0x00000301;
pub const MSR_P4_BPU_PERFCTR2: ::std::os::raw::c_uint = 0x00000302;
pub const MSR_P4_BPU_PERFCTR3: ::std::os::raw::c_uint = 0x00000303;
pub const MSR_P4_MS_PERFCTR0: ::std::os::raw::c_uint = 0x00000304;
pub const MSR_P4_MS_PERFCTR1: ::std::os::raw::c_uint = 0x00000305;
pub const MSR_P4_MS_PERFCTR2: ::std::os::raw::c_uint = 0x00000306;
pub const MSR_P4_MS_PERFCTR3: ::std::os::raw::c_uint = 0x00000307;
pub const MSR_P4_FLAME_PERFCTR0: ::std::os::raw::c_uint = 0x00000308;
pub const MSR_P4_FLAME_PERFCTR1: ::std::os::raw::c_uint = 0x00000309;
pub const MSR_P4_FLAME_PERFCTR2: ::std::os::raw::c_uint = 0x0000030a;
pub const MSR_P4_FLAME_PERFCTR3: ::std::os::raw::c_uint = 0x0000030b;
pub const MSR_P4_IQ_PERFCTR0: ::std::os::raw::c_uint = 0x0000030c;
pub const MSR_P4_IQ_PERFCTR1: ::std::os::raw::c_uint = 0x0000030d;
pub const MSR_P4_IQ_PERFCTR2: ::std::os::raw::c_uint = 0x0000030e;
pub const MSR_P4_IQ_PERFCTR3: ::std::os::raw::c_uint = 0x0000030f;
pub const MSR_P4_IQ_PERFCTR4: ::std::os::raw::c_uint = 0x00000310;
pub const MSR_P4_IQ_PERFCTR5: ::std::os::raw::c_uint = 0x00000311;
pub const MSR_P4_BPU_CCCR0: ::std::os::raw::c_uint = 0x00000360;
pub const MSR_P4_BPU_CCCR1: ::std::os::raw::c_uint = 0x00000361;
pub const MSR_P4_BPU_CCCR2: ::std::os::raw::c_uint = 0x00000362;
pub const MSR_P4_BPU_CCCR3: ::std::os::raw::c_uint = 0x00000363;
pub const MSR_P4_MS_CCCR0: ::std::os::raw::c_uint = 0x00000364;
pub const MSR_P4_MS_CCCR1: ::std::os::raw::c_uint = 0x00000365;
pub const MSR_P4_MS_CCCR2: ::std::os::raw::c_uint = 0x00000366;
pub const MSR_P4_MS_CCCR3: ::std::os::raw::c_uint = 0x00000367;
pub const MSR_P4_FLAME_CCCR0: ::std::os::raw::c_uint = 0x00000368;
pub const MSR_P4_FLAME_CCCR1: ::std::os::raw::c_uint = 0x00000369;
pub const MSR_P4_FLAME_CCCR2: ::std::os::raw::c_uint = 0x0000036a;
pub const MSR_P4_FLAME_CCCR3: ::std::os::raw::c_uint = 0x0000036b;
pub const MSR_P4_IQ_CCCR0: ::std::os::raw::c_uint = 0x0000036c;
pub const MSR_P4_IQ_CCCR1: ::std::os::raw::c_uint = 0x0000036d;
pub const MSR_P4_IQ_CCCR2: ::std::os::raw::c_uint = 0x0000036e;
pub const MSR_P4_IQ_CCCR3: ::std::os::raw::c_uint = 0x0000036f;
pub const MSR_P4_IQ_CCCR4: ::std::os::raw::c_uint = 0x00000370;
pub const MSR_P4_IQ_CCCR5: ::std::os::raw::c_uint = 0x00000371;
pub const MSR_P4_ALF_ESCR0: ::std::os::raw::c_uint = 0x000003ca;
pub const MSR_P4_ALF_ESCR1: ::std::os::raw::c_uint = 0x000003cb;
pub const MSR_P4_BPU_ESCR0: ::std::os::raw::c_uint = 0x000003b2;
pub const MSR_P4_BPU_ESCR1: ::std::os::raw::c_uint = 0x000003b3;
pub const MSR_P4_BSU_ESCR0: ::std::os::raw::c_uint = 0x000003a0;
pub const MSR_P4_BSU_ESCR1: ::std::os::raw::c_uint = 0x000003a1;
pub const MSR_P4_CRU_ESCR0: ::std::os::raw::c_uint = 0x000003b8;
pub const MSR_P4_CRU_ESCR1: ::std::os::raw::c_uint = 0x000003b9;
pub const MSR_P4_CRU_ESCR2: ::std::os::raw::c_uint = 0x000003cc;
pub const MSR_P4_CRU_ESCR3: ::std::os::raw::c_uint = 0x000003cd;
pub const MSR_P4_CRU_ESCR4: ::std::os::raw::c_uint = 0x000003e0;
pub const MSR_P4_CRU_ESCR5: ::std::os::raw::c_uint = 0x000003e1;
pub const MSR_P4_DAC_ESCR0: ::std::os::raw::c_uint = 0x000003a8;
pub const MSR_P4_DAC_ESCR1: ::std::os::raw::c_uint = 0x000003a9;
pub const MSR_P4_FIRM_ESCR0: ::std::os::raw::c_uint = 0x000003a4;
pub const MSR_P4_FIRM_ESCR1: ::std::os::raw::c_uint = 0x000003a5;
pub const MSR_P4_FLAME_ESCR0: ::std::os::raw::c_uint = 0x000003a6;
pub const MSR_P4_FLAME_ESCR1: ::std::os::raw::c_uint = 0x000003a7;
pub const MSR_P4_FSB_ESCR0: ::std::os::raw::c_uint = 0x000003a2;
pub const MSR_P4_FSB_ESCR1: ::std::os::raw::c_uint = 0x000003a3;
pub const MSR_P4_IQ_ESCR0: ::std::os::raw::c_uint = 0x000003ba;
pub const MSR_P4_IQ_ESCR1: ::std::os::raw::c_uint = 0x000003bb;
pub const MSR_P4_IS_ESCR0: ::std::os::raw::c_uint = 0x000003b4;
pub const MSR_P4_IS_ESCR1: ::std::os::raw::c_uint = 0x000003b5;
pub const MSR_P4_ITLB_ESCR0: ::std::os::raw::c_uint = 0x000003b6;
pub const MSR_P4_ITLB_ESCR1: ::std::os::raw::c_uint = 0x000003b7;
pub const MSR_P4_IX_ESCR0: ::std::os::raw::c_uint = 0x000003c8;
pub const MSR_P4_IX_ESCR1: ::std::os::raw::c_uint = 0x000003c9;
pub const MSR_P4_MOB_ESCR0: ::std::os::raw::c_uint = 0x000003aa;
pub const MSR_P4_MOB_ESCR1: ::std::os::raw::c_uint = 0x000003ab;
pub const MSR_P4_MS_ESCR0: ::std::os::raw::c_uint = 0x000003c0;
pub const MSR_P4_MS_ESCR1: ::std::os::raw::c_uint = 0x000003c1;
pub const MSR_P4_PMH_ESCR0: ::std::os::raw::c_uint = 0x000003ac;
pub const MSR_P4_PMH_ESCR1: ::std::os::raw::c_uint = 0x000003ad;
pub const MSR_P4_RAT_ESCR0: ::std::os::raw::c_uint = 0x000003bc;
pub const MSR_P4_RAT_ESCR1: ::std::os::raw::c_uint = 0x000003bd;
pub const MSR_P4_SAAT_ESCR0: ::std::os::raw::c_uint = 0x000003ae;
pub const MSR_P4_SAAT_ESCR1: ::std::os::raw::c_uint = 0x000003af;
pub const MSR_P4_SSU_ESCR0: ::std::os::raw::c_uint = 0x000003be;
pub const MSR_P4_SSU_ESCR1: ::std::os::raw::c_uint = 0x000003bf;
pub const MSR_P4_TBPU_ESCR0: ::std::os::raw::c_uint = 0x000003c2;
pub const MSR_P4_TBPU_ESCR1: ::std::os::raw::c_uint = 0x000003c3;
pub const MSR_P4_TC_ESCR0: ::std::os::raw::c_uint = 0x000003c4;
pub const MSR_P4_TC_ESCR1: ::std::os::raw::c_uint = 0x000003c5;
pub const MSR_P4_U2L_ESCR0: ::std::os::raw::c_uint = 0x000003b0;
pub const MSR_P4_U2L_ESCR1: ::std::os::raw::c_uint = 0x000003b1;
pub const MSR_P4_PEBS_MATRIX_VERT: ::std::os::raw::c_uint = 0x000003f2;
pub const MSR_CORE_PERF_FIXED_CTR0: ::std::os::raw::c_uint = 0x00000309;
pub const MSR_CORE_PERF_FIXED_CTR1: ::std::os::raw::c_uint = 0x0000030a;
pub const MSR_CORE_PERF_FIXED_CTR2: ::std::os::raw::c_uint = 0x0000030b;
pub const MSR_CORE_PERF_FIXED_CTR_CTRL: ::std::os::raw::c_uint = 0x0000038d;
pub const MSR_CORE_PERF_GLOBAL_STATUS: ::std::os::raw::c_uint = 0x0000038e;
pub const MSR_CORE_PERF_GLOBAL_CTRL: ::std::os::raw::c_uint = 0x0000038f;
pub const MSR_CORE_PERF_GLOBAL_OVF_CTRL: ::std::os::raw::c_uint = 0x00000390;
pub const MSR_GEODE_BUSCONT_CONF0: ::std::os::raw::c_uint = 0x00001900;
pub const MSR_IA32_VMX_BASIC: ::std::os::raw::c_uint = 0x00000480;
pub const MSR_IA32_VMX_PINBASED_CTLS: ::std::os::raw::c_uint = 0x00000481;
pub const MSR_IA32_VMX_PROCBASED_CTLS: ::std::os::raw::c_uint = 0x00000482;
pub const MSR_IA32_VMX_EXIT_CTLS: ::std::os::raw::c_uint = 0x00000483;
pub const MSR_IA32_VMX_ENTRY_CTLS: ::std::os::raw::c_uint = 0x00000484;
pub const MSR_IA32_VMX_MISC: ::std::os::raw::c_uint = 0x00000485;
pub const MSR_IA32_VMX_CR0_FIXED0: ::std::os::raw::c_uint = 0x00000486;
pub const MSR_IA32_VMX_CR0_FIXED1: ::std::os::raw::c_uint = 0x00000487;
pub const MSR_IA32_VMX_CR4_FIXED0: ::std::os::raw::c_uint = 0x00000488;
pub const MSR_IA32_VMX_CR4_FIXED1: ::std::os::raw::c_uint = 0x00000489;
pub const MSR_IA32_VMX_VMCS_ENUM: ::std::os::raw::c_uint = 0x0000048a;
pub const MSR_IA32_VMX_PROCBASED_CTLS2: ::std::os::raw::c_uint = 0x0000048b;
pub const MSR_IA32_VMX_EPT_VPID_CAP: ::std::os::raw::c_uint = 0x0000048c;
pub const MSR_IA32_VMX_TRUE_PINBASED_CTLS: ::std::os::raw::c_uint = 0x0000048d;
pub const MSR_IA32_VMX_TRUE_PROCBASED_CTLS: ::std::os::raw::c_uint = 0x0000048e;
pub const MSR_IA32_VMX_TRUE_EXIT_CTLS: ::std::os::raw::c_uint = 0x0000048f;
pub const MSR_IA32_VMX_TRUE_ENTRY_CTLS: ::std::os::raw::c_uint = 0x00000490;
pub const MSR_IA32_VMX_VMFUNC: ::std::os::raw::c_uint = 0x00000491;
pub const VMX_BASIC_VMCS_SIZE_SHIFT: ::std::os::raw::c_uint = 0x00000020;
pub const VMX_BASIC_TRUE_CTLS: ::std::os::raw::c_ulonglong = 0x80000000000000;
pub const VMX_BASIC_64: ::std::os::raw::c_ulonglong = 0x1000000000000;
pub const VMX_BASIC_MEM_TYPE_SHIFT: ::std::os::raw::c_uint = 0x00000032;
pub const VMX_BASIC_MEM_TYPE_MASK: ::std::os::raw::c_ulonglong = 0x3c000000000000;
pub const VMX_BASIC_MEM_TYPE_WB: ::std::os::raw::c_uint = 0x00000006;
pub const VMX_BASIC_INOUT: ::std::os::raw::c_ulonglong = 0x40000000000000;
pub const MSR_IA32_VMX_MISC_VMWRITE_SHADOW_RO_FIELDS: ::std::os::raw::c_uint = 0x20000000;
pub const MSR_IA32_VMX_MISC_PREEMPTION_TIMER_SCALE: ::std::os::raw::c_uint = 0x0000001f;
pub const MSR_VM_CR: ::std::os::raw::c_uint = 0xc0010114;
pub const MSR_VM_IGNNE: ::std::os::raw::c_uint = 0xc0010115;
pub const MSR_VM_HSAVE_PA: ::std::os::raw::c_uint = 0xc0010117;
