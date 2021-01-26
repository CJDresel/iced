// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

namespace Generator.Enums {
	[Enum("Mnemonic", Documentation = "Mnemonic", Public = true)]
	enum Mnemonic {
		INVALID,
		Aaa,
		Aad,
		Aam,
		Aas,
		Adc,
		Adcx,
		Add,
		Addpd,
		Addps,
		Addsd,
		Addss,
		Addsubpd,
		Addsubps,
		Adox,
		Aesdec,
		Aesdeclast,
		Aesenc,
		Aesenclast,
		Aesimc,
		Aeskeygenassist,
		And,
		Andn,
		Andnpd,
		Andnps,
		Andpd,
		Andps,
		Arpl,
		Bextr,
		Blcfill,
		Blci,
		Blcic,
		Blcmsk,
		Blcs,
		Blendpd,
		Blendps,
		Blendvpd,
		Blendvps,
		Blsfill,
		Blsi,
		Blsic,
		Blsmsk,
		Blsr,
		Bndcl,
		Bndcn,
		Bndcu,
		Bndldx,
		Bndmk,
		Bndmov,
		Bndstx,
		Bound,
		Bsf,
		Bsr,
		Bswap,
		Bt,
		Btc,
		Btr,
		Bts,
		Bzhi,
		Call,
		Cbw,
		Cdq,
		Cdqe,
		Cl1invmb,
		Clac,
		Clc,
		Cld,
		Cldemote,
		Clflush,
		Clflushopt,
		Clgi,
		Cli,
		Clrssbsy,
		Clts,
		Clwb,
		Clzero,
		Cmc,
		Cmova,
		Cmovae,
		Cmovb,
		Cmovbe,
		Cmove,
		Cmovg,
		Cmovge,
		Cmovl,
		Cmovle,
		Cmovne,
		Cmovno,
		Cmovnp,
		Cmovns,
		Cmovo,
		Cmovp,
		Cmovs,
		Cmp,
		Cmppd,
		Cmpps,
		Cmpsb,
		Cmpsd,
		Cmpsq,
		Cmpss,
		Cmpsw,
		Cmpxchg,
		Cmpxchg16b,
		Cmpxchg8b,
		Comisd,
		Comiss,
		Cpuid,
		Cqo,
		Crc32,
		Cvtdq2pd,
		Cvtdq2ps,
		Cvtpd2dq,
		Cvtpd2pi,
		Cvtpd2ps,
		Cvtpi2pd,
		Cvtpi2ps,
		Cvtps2dq,
		Cvtps2pd,
		Cvtps2pi,
		Cvtsd2si,
		Cvtsd2ss,
		Cvtsi2sd,
		Cvtsi2ss,
		Cvtss2sd,
		Cvtss2si,
		Cvttpd2dq,
		Cvttpd2pi,
		Cvttps2dq,
		Cvttps2pi,
		Cvttsd2si,
		Cvttss2si,
		Cwd,
		Cwde,
		Daa,
		Das,
		Db,
		Dd,
		Dec,
		Div,
		Divpd,
		Divps,
		Divsd,
		Divss,
		Dppd,
		Dpps,
		Dq,
		Dw,
		Emms,
		Encls,
		Enclu,
		Enclv,
		Endbr32,
		Endbr64,
		Enqcmd,
		Enqcmds,
		Enter,
		Extractps,
		Extrq,
		F2xm1,
		Fabs,
		Fadd,
		Faddp,
		Fbld,
		Fbstp,
		Fchs,
		Fclex,
		Fcmovb,
		Fcmovbe,
		Fcmove,
		Fcmovnb,
		Fcmovnbe,
		Fcmovne,
		Fcmovnu,
		Fcmovu,
		Fcom,
		Fcomi,
		Fcomip,
		Fcomp,
		Fcompp,
		Fcos,
		Fdecstp,
		Fdisi,
		Fdiv,
		Fdivp,
		Fdivr,
		Fdivrp,
		Femms,
		Feni,
		Ffree,
		Ffreep,
		Fiadd,
		Ficom,
		Ficomp,
		Fidiv,
		Fidivr,
		Fild,
		Fimul,
		Fincstp,
		Finit,
		Fist,
		Fistp,
		Fisttp,
		Fisub,
		Fisubr,
		Fld,
		Fld1,
		Fldcw,
		Fldenv,
		Fldl2e,
		Fldl2t,
		Fldlg2,
		Fldln2,
		Fldpi,
		Fldz,
		Fmul,
		Fmulp,
		Fnclex,
		Fndisi,
		Fneni,
		Fninit,
		Fnop,
		Fnsave,
		Fnsetpm,
		Fnstcw,
		Fnstenv,
		Fnstsw,
		Fpatan,
		Fprem,
		Fprem1,
		Fptan,
		Frndint,
		Frstor,
		Frstpm,
		Fsave,
		Fscale,
		Fsetpm,
		Fsin,
		Fsincos,
		Fsqrt,
		Fst,
		Fstcw,
		Fstdw,
		Fstenv,
		Fstp,
		Fstpnce,
		Fstsg,
		Fstsw,
		Fsub,
		Fsubp,
		Fsubr,
		Fsubrp,
		Ftst,
		Fucom,
		Fucomi,
		Fucomip,
		Fucomp,
		Fucompp,
		Fxam,
		Fxch,
		Fxrstor,
		Fxrstor64,
		Fxsave,
		Fxsave64,
		Fxtract,
		Fyl2x,
		Fyl2xp1,
		Getsec,
		Gf2p8affineinvqb,
		Gf2p8affineqb,
		Gf2p8mulb,
		Haddpd,
		Haddps,
		Hlt,
		Hsubpd,
		Hsubps,
		Ibts,
		Idiv,
		Imul,
		In,
		Inc,
		Incsspd,
		Incsspq,
		Insb,
		Insd,
		Insertps,
		Insertq,
		Insw,
		Int,
		Int1,
		Into,
		Invd,
		Invept,
		Invlpg,
		Invlpga,
		Invpcid,
		Invvpid,
		Iret,
		Ja,
		Jae,
		Jb,
		Jbe,
		Jcxz,
		Je,
		Jecxz,
		Jg,
		Jge,
		Jl,
		Jle,
		Jmp,
		Jmpe,
		Jne,
		Jno,
		Jnp,
		Jns,
		Jo,
		Jp,
		Jrcxz,
		Js,
		Kaddb,
		Kaddd,
		Kaddq,
		Kaddw,
		Kandb,
		Kandd,
		Kandnb,
		Kandnd,
		Kandnq,
		Kandnw,
		Kandq,
		Kandw,
		Kmovb,
		Kmovd,
		Kmovq,
		Kmovw,
		Knotb,
		Knotd,
		Knotq,
		Knotw,
		Korb,
		Kord,
		Korq,
		Kortestb,
		Kortestd,
		Kortestq,
		Kortestw,
		Korw,
		Kshiftlb,
		Kshiftld,
		Kshiftlq,
		Kshiftlw,
		Kshiftrb,
		Kshiftrd,
		Kshiftrq,
		Kshiftrw,
		Ktestb,
		Ktestd,
		Ktestq,
		Ktestw,
		Kunpckbw,
		Kunpckdq,
		Kunpckwd,
		Kxnorb,
		Kxnord,
		Kxnorq,
		Kxnorw,
		Kxorb,
		Kxord,
		Kxorq,
		Kxorw,
		Lahf,
		Lar,
		Lddqu,
		Ldmxcsr,
		Lds,
		Lea,
		Leave,
		Les,
		Lfence,
		Lfs,
		Lgdt,
		Lgs,
		Lidt,
		Lldt,
		Llwpcb,
		Lmsw,
		Loadall,
		Lodsb,
		Lodsd,
		Lodsq,
		Lodsw,
		Loop,
		Loope,
		Loopne,
		Lsl,
		Lss,
		Ltr,
		Lwpins,
		Lwpval,
		Lzcnt,
		Maskmovdqu,
		Maskmovq,
		Maxpd,
		Maxps,
		Maxsd,
		Maxss,
		Mcommit,
		Mfence,
		Minpd,
		Minps,
		Minsd,
		Minss,
		Monitor,
		Monitorx,
		Montmul,
		Mov,
		Movapd,
		Movaps,
		Movbe,
		Movd,
		Movddup,
		Movdir64b,
		Movdiri,
		Movdq2q,
		Movdqa,
		Movdqu,
		Movhlps,
		Movhpd,
		Movhps,
		Movlhps,
		Movlpd,
		Movlps,
		Movmskpd,
		Movmskps,
		Movntdq,
		Movntdqa,
		Movnti,
		Movntpd,
		Movntps,
		Movntq,
		Movntsd,
		Movntss,
		Movq,
		Movq2dq,
		Movsb,
		Movsd,
		Movshdup,
		Movsldup,
		Movsq,
		Movss,
		Movsw,
		Movsx,
		Movsxd,
		Movupd,
		Movups,
		Movzx,
		Mpsadbw,
		Mul,
		Mulpd,
		Mulps,
		Mulsd,
		Mulss,
		Mulx,
		Mwait,
		Mwaitx,
		Neg,
		Nop,
		Not,
		Or,
		Orpd,
		Orps,
		Out,
		Outsb,
		Outsd,
		Outsw,
		Pabsb,
		Pabsd,
		Pabsw,
		Packssdw,
		Packsswb,
		Packusdw,
		Packuswb,
		Paddb,
		Paddd,
		Paddq,
		Paddsb,
		Paddsw,
		Paddusb,
		Paddusw,
		Paddw,
		Palignr,
		Pand,
		Pandn,
		Pause,
		Pavgb,
		Pavgusb,
		Pavgw,
		Pblendvb,
		Pblendw,
		Pclmulqdq,
		Pcmpeqb,
		Pcmpeqd,
		Pcmpeqq,
		Pcmpeqw,
		Pcmpestri,
		Pcmpestri64,
		Pcmpestrm,
		Pcmpestrm64,
		Pcmpgtb,
		Pcmpgtd,
		Pcmpgtq,
		Pcmpgtw,
		Pcmpistri,
		Pcmpistrm,
		Pcommit,
		Pconfig,
		Pdep,
		Pext,
		Pextrb,
		Pextrd,
		Pextrq,
		Pextrw,
		Pf2id,
		Pf2iw,
		Pfacc,
		Pfadd,
		Pfcmpeq,
		Pfcmpge,
		Pfcmpgt,
		Pfmax,
		Pfmin,
		Pfmul,
		Pfnacc,
		Pfpnacc,
		Pfrcp,
		Pfrcpit1,
		Pfrcpit2,
		Pfrcpv,
		Pfrsqit1,
		Pfrsqrt,
		Pfrsqrtv,
		Pfsub,
		Pfsubr,
		Phaddd,
		Phaddsw,
		Phaddw,
		Phminposuw,
		Phsubd,
		Phsubsw,
		Phsubw,
		Pi2fd,
		Pi2fw,
		Pinsrb,
		Pinsrd,
		Pinsrq,
		Pinsrw,
		Pmaddubsw,
		Pmaddwd,
		Pmaxsb,
		Pmaxsd,
		Pmaxsw,
		Pmaxub,
		Pmaxud,
		Pmaxuw,
		Pminsb,
		Pminsd,
		Pminsw,
		Pminub,
		Pminud,
		Pminuw,
		Pmovmskb,
		Pmovsxbd,
		Pmovsxbq,
		Pmovsxbw,
		Pmovsxdq,
		Pmovsxwd,
		Pmovsxwq,
		Pmovzxbd,
		Pmovzxbq,
		Pmovzxbw,
		Pmovzxdq,
		Pmovzxwd,
		Pmovzxwq,
		Pmuldq,
		Pmulhrsw,
		Pmulhrw,
		Pmulhuw,
		Pmulhw,
		Pmulld,
		Pmullw,
		Pmuludq,
		Pop,
		Popa,
		Popcnt,
		Popf,
		Por,
		Prefetch,
		Prefetchnta,
		Prefetcht0,
		Prefetcht1,
		Prefetcht2,
		Prefetchw,
		Prefetchwt1,
		Psadbw,
		Pshufb,
		Pshufd,
		Pshufhw,
		Pshuflw,
		Pshufw,
		Psignb,
		Psignd,
		Psignw,
		Pslld,
		Pslldq,
		Psllq,
		Psllw,
		Psrad,
		Psraw,
		Psrld,
		Psrldq,
		Psrlq,
		Psrlw,
		Psubb,
		Psubd,
		Psubq,
		Psubsb,
		Psubsw,
		Psubusb,
		Psubusw,
		Psubw,
		Pswapd,
		Ptest,
		Ptwrite,
		Punpckhbw,
		Punpckhdq,
		Punpckhqdq,
		Punpckhwd,
		Punpcklbw,
		Punpckldq,
		Punpcklqdq,
		Punpcklwd,
		Push,
		Pusha,
		Pushf,
		Pxor,
		Rcl,
		Rcpps,
		Rcpss,
		Rcr,
		Rdfsbase,
		Rdgsbase,
		Rdmsr,
		Rdpid,
		Rdpkru,
		Rdpmc,
		Rdpru,
		Rdrand,
		Rdseed,
		Rdsspd,
		Rdsspq,
		Rdtsc,
		Rdtscp,
		Reservednop,
		Ret,
		Retf,
		Rol,
		Ror,
		Rorx,
		Roundpd,
		Roundps,
		Roundsd,
		Roundss,
		Rsm,
		Rsqrtps,
		Rsqrtss,
		Rstorssp,
		Sahf,
		Sal,
		Salc,
		Sar,
		Sarx,
		Saveprevssp,
		Sbb,
		Scasb,
		Scasd,
		Scasq,
		Scasw,
		Seta,
		Setae,
		Setb,
		Setbe,
		Sete,
		Setg,
		Setge,
		Setl,
		Setle,
		Setne,
		Setno,
		Setnp,
		Setns,
		Seto,
		Setp,
		Sets,
		Setssbsy,
		Sfence,
		Sgdt,
		Sha1msg1,
		Sha1msg2,
		Sha1nexte,
		Sha1rnds4,
		Sha256msg1,
		Sha256msg2,
		Sha256rnds2,
		Shl,
		Shld,
		Shlx,
		Shr,
		Shrd,
		Shrx,
		Shufpd,
		Shufps,
		Sidt,
		Skinit,
		Sldt,
		Slwpcb,
		Smsw,
		Sqrtpd,
		Sqrtps,
		Sqrtsd,
		Sqrtss,
		Stac,
		Stc,
		Std,
		Stgi,
		Sti,
		Stmxcsr,
		Stosb,
		Stosd,
		Stosq,
		Stosw,
		Str,
		Sub,
		Subpd,
		Subps,
		Subsd,
		Subss,
		Swapgs,
		Syscall,
		Sysenter,
		Sysexit,
		Sysret,
		T1mskc,
		Test,
		Tpause,
		Tzcnt,
		Tzmsk,
		Ucomisd,
		Ucomiss,
		Ud0,
		Ud1,
		Ud2,
		Umonitor,
		Umov,
		Umwait,
		Unpckhpd,
		Unpckhps,
		Unpcklpd,
		Unpcklps,
		V4fmaddps,
		V4fmaddss,
		V4fnmaddps,
		V4fnmaddss,
		Vaddpd,
		Vaddps,
		Vaddsd,
		Vaddss,
		Vaddsubpd,
		Vaddsubps,
		Vaesdec,
		Vaesdeclast,
		Vaesenc,
		Vaesenclast,
		Vaesimc,
		Vaeskeygenassist,
		Valignd,
		Valignq,
		Vandnpd,
		Vandnps,
		Vandpd,
		Vandps,
		Vblendmpd,
		Vblendmps,
		Vblendpd,
		Vblendps,
		Vblendvpd,
		Vblendvps,
		Vbroadcastf128,
		Vbroadcastf32x2,
		Vbroadcastf32x4,
		Vbroadcastf32x8,
		Vbroadcastf64x2,
		Vbroadcastf64x4,
		Vbroadcasti128,
		Vbroadcasti32x2,
		Vbroadcasti32x4,
		Vbroadcasti32x8,
		Vbroadcasti64x2,
		Vbroadcasti64x4,
		Vbroadcastsd,
		Vbroadcastss,
		Vcmppd,
		Vcmpps,
		Vcmpsd,
		Vcmpss,
		Vcomisd,
		Vcomiss,
		Vcompresspd,
		Vcompressps,
		Vcvtdq2pd,
		Vcvtdq2ps,
		Vcvtne2ps2bf16,
		Vcvtneps2bf16,
		Vcvtpd2dq,
		Vcvtpd2ps,
		Vcvtpd2qq,
		Vcvtpd2udq,
		Vcvtpd2uqq,
		Vcvtph2ps,
		Vcvtps2dq,
		Vcvtps2pd,
		Vcvtps2ph,
		Vcvtps2qq,
		Vcvtps2udq,
		Vcvtps2uqq,
		Vcvtqq2pd,
		Vcvtqq2ps,
		Vcvtsd2si,
		Vcvtsd2ss,
		Vcvtsd2usi,
		Vcvtsi2sd,
		Vcvtsi2ss,
		Vcvtss2sd,
		Vcvtss2si,
		Vcvtss2usi,
		Vcvttpd2dq,
		Vcvttpd2qq,
		Vcvttpd2udq,
		Vcvttpd2uqq,
		Vcvttps2dq,
		Vcvttps2qq,
		Vcvttps2udq,
		Vcvttps2uqq,
		Vcvttsd2si,
		Vcvttsd2usi,
		Vcvttss2si,
		Vcvttss2usi,
		Vcvtudq2pd,
		Vcvtudq2ps,
		Vcvtuqq2pd,
		Vcvtuqq2ps,
		Vcvtusi2sd,
		Vcvtusi2ss,
		Vdbpsadbw,
		Vdivpd,
		Vdivps,
		Vdivsd,
		Vdivss,
		Vdpbf16ps,
		Vdppd,
		Vdpps,
		Verr,
		Verw,
		Vexp2pd,
		Vexp2ps,
		Vexpandpd,
		Vexpandps,
		Vextractf128,
		Vextractf32x4,
		Vextractf32x8,
		Vextractf64x2,
		Vextractf64x4,
		Vextracti128,
		Vextracti32x4,
		Vextracti32x8,
		Vextracti64x2,
		Vextracti64x4,
		Vextractps,
		Vfixupimmpd,
		Vfixupimmps,
		Vfixupimmsd,
		Vfixupimmss,
		Vfmadd132pd,
		Vfmadd132ps,
		Vfmadd132sd,
		Vfmadd132ss,
		Vfmadd213pd,
		Vfmadd213ps,
		Vfmadd213sd,
		Vfmadd213ss,
		Vfmadd231pd,
		Vfmadd231ps,
		Vfmadd231sd,
		Vfmadd231ss,
		Vfmaddpd,
		Vfmaddps,
		Vfmaddsd,
		Vfmaddss,
		Vfmaddsub132pd,
		Vfmaddsub132ps,
		Vfmaddsub213pd,
		Vfmaddsub213ps,
		Vfmaddsub231pd,
		Vfmaddsub231ps,
		Vfmaddsubpd,
		Vfmaddsubps,
		Vfmsub132pd,
		Vfmsub132ps,
		Vfmsub132sd,
		Vfmsub132ss,
		Vfmsub213pd,
		Vfmsub213ps,
		Vfmsub213sd,
		Vfmsub213ss,
		Vfmsub231pd,
		Vfmsub231ps,
		Vfmsub231sd,
		Vfmsub231ss,
		Vfmsubadd132pd,
		Vfmsubadd132ps,
		Vfmsubadd213pd,
		Vfmsubadd213ps,
		Vfmsubadd231pd,
		Vfmsubadd231ps,
		Vfmsubaddpd,
		Vfmsubaddps,
		Vfmsubpd,
		Vfmsubps,
		Vfmsubsd,
		Vfmsubss,
		Vfnmadd132pd,
		Vfnmadd132ps,
		Vfnmadd132sd,
		Vfnmadd132ss,
		Vfnmadd213pd,
		Vfnmadd213ps,
		Vfnmadd213sd,
		Vfnmadd213ss,
		Vfnmadd231pd,
		Vfnmadd231ps,
		Vfnmadd231sd,
		Vfnmadd231ss,
		Vfnmaddpd,
		Vfnmaddps,
		Vfnmaddsd,
		Vfnmaddss,
		Vfnmsub132pd,
		Vfnmsub132ps,
		Vfnmsub132sd,
		Vfnmsub132ss,
		Vfnmsub213pd,
		Vfnmsub213ps,
		Vfnmsub213sd,
		Vfnmsub213ss,
		Vfnmsub231pd,
		Vfnmsub231ps,
		Vfnmsub231sd,
		Vfnmsub231ss,
		Vfnmsubpd,
		Vfnmsubps,
		Vfnmsubsd,
		Vfnmsubss,
		Vfpclasspd,
		Vfpclassps,
		Vfpclasssd,
		Vfpclassss,
		Vfrczpd,
		Vfrczps,
		Vfrczsd,
		Vfrczss,
		Vgatherdpd,
		Vgatherdps,
		Vgatherpf0dpd,
		Vgatherpf0dps,
		Vgatherpf0qpd,
		Vgatherpf0qps,
		Vgatherpf1dpd,
		Vgatherpf1dps,
		Vgatherpf1qpd,
		Vgatherpf1qps,
		Vgatherqpd,
		Vgatherqps,
		Vgetexppd,
		Vgetexpps,
		Vgetexpsd,
		Vgetexpss,
		Vgetmantpd,
		Vgetmantps,
		Vgetmantsd,
		Vgetmantss,
		Vgf2p8affineinvqb,
		Vgf2p8affineqb,
		Vgf2p8mulb,
		Vhaddpd,
		Vhaddps,
		Vhsubpd,
		Vhsubps,
		Vinsertf128,
		Vinsertf32x4,
		Vinsertf32x8,
		Vinsertf64x2,
		Vinsertf64x4,
		Vinserti128,
		Vinserti32x4,
		Vinserti32x8,
		Vinserti64x2,
		Vinserti64x4,
		Vinsertps,
		Vlddqu,
		Vldmxcsr,
		Vmaskmovdqu,
		Vmaskmovpd,
		Vmaskmovps,
		Vmaxpd,
		Vmaxps,
		Vmaxsd,
		Vmaxss,
		Vmcall,
		Vmclear,
		Vmfunc,
		Vminpd,
		Vminps,
		Vminsd,
		Vminss,
		Vmlaunch,
		Vmload,
		Vmmcall,
		Vmovapd,
		Vmovaps,
		Vmovd,
		Vmovddup,
		Vmovdqa,
		Vmovdqa32,
		Vmovdqa64,
		Vmovdqu,
		Vmovdqu16,
		Vmovdqu32,
		Vmovdqu64,
		Vmovdqu8,
		Vmovhlps,
		Vmovhpd,
		Vmovhps,
		Vmovlhps,
		Vmovlpd,
		Vmovlps,
		Vmovmskpd,
		Vmovmskps,
		Vmovntdq,
		Vmovntdqa,
		Vmovntpd,
		Vmovntps,
		Vmovq,
		Vmovsd,
		Vmovshdup,
		Vmovsldup,
		Vmovss,
		Vmovupd,
		Vmovups,
		Vmpsadbw,
		Vmptrld,
		Vmptrst,
		Vmread,
		Vmresume,
		Vmrun,
		Vmsave,
		Vmulpd,
		Vmulps,
		Vmulsd,
		Vmulss,
		Vmwrite,
		Vmxoff,
		Vmxon,
		Vorpd,
		Vorps,
		Vp2intersectd,
		Vp2intersectq,
		Vp4dpwssd,
		Vp4dpwssds,
		Vpabsb,
		Vpabsd,
		Vpabsq,
		Vpabsw,
		Vpackssdw,
		Vpacksswb,
		Vpackusdw,
		Vpackuswb,
		Vpaddb,
		Vpaddd,
		Vpaddq,
		Vpaddsb,
		Vpaddsw,
		Vpaddusb,
		Vpaddusw,
		Vpaddw,
		Vpalignr,
		Vpand,
		Vpandd,
		Vpandn,
		Vpandnd,
		Vpandnq,
		Vpandq,
		Vpavgb,
		Vpavgw,
		Vpblendd,
		Vpblendmb,
		Vpblendmd,
		Vpblendmq,
		Vpblendmw,
		Vpblendvb,
		Vpblendw,
		Vpbroadcastb,
		Vpbroadcastd,
		Vpbroadcastmb2q,
		Vpbroadcastmw2d,
		Vpbroadcastq,
		Vpbroadcastw,
		Vpclmulqdq,
		Vpcmov,
		Vpcmpb,
		Vpcmpd,
		Vpcmpeqb,
		Vpcmpeqd,
		Vpcmpeqq,
		Vpcmpeqw,
		Vpcmpestri,
		Vpcmpestri64,
		Vpcmpestrm,
		Vpcmpestrm64,
		Vpcmpgtb,
		Vpcmpgtd,
		Vpcmpgtq,
		Vpcmpgtw,
		Vpcmpistri,
		Vpcmpistrm,
		Vpcmpq,
		Vpcmpub,
		Vpcmpud,
		Vpcmpuq,
		Vpcmpuw,
		Vpcmpw,
		Vpcomb,
		Vpcomd,
		Vpcompressb,
		Vpcompressd,
		Vpcompressq,
		Vpcompressw,
		Vpcomq,
		Vpcomub,
		Vpcomud,
		Vpcomuq,
		Vpcomuw,
		Vpcomw,
		Vpconflictd,
		Vpconflictq,
		Vpdpbusd,
		Vpdpbusds,
		Vpdpwssd,
		Vpdpwssds,
		Vperm2f128,
		Vperm2i128,
		Vpermb,
		Vpermd,
		Vpermi2b,
		Vpermi2d,
		Vpermi2pd,
		Vpermi2ps,
		Vpermi2q,
		Vpermi2w,
		Vpermil2pd,
		Vpermil2ps,
		Vpermilpd,
		Vpermilps,
		Vpermpd,
		Vpermps,
		Vpermq,
		Vpermt2b,
		Vpermt2d,
		Vpermt2pd,
		Vpermt2ps,
		Vpermt2q,
		Vpermt2w,
		Vpermw,
		Vpexpandb,
		Vpexpandd,
		Vpexpandq,
		Vpexpandw,
		Vpextrb,
		Vpextrd,
		Vpextrq,
		Vpextrw,
		Vpgatherdd,
		Vpgatherdq,
		Vpgatherqd,
		Vpgatherqq,
		Vphaddbd,
		Vphaddbq,
		Vphaddbw,
		Vphaddd,
		Vphadddq,
		Vphaddsw,
		Vphaddubd,
		Vphaddubq,
		Vphaddubw,
		Vphaddudq,
		Vphadduwd,
		Vphadduwq,
		Vphaddw,
		Vphaddwd,
		Vphaddwq,
		Vphminposuw,
		Vphsubbw,
		Vphsubd,
		Vphsubdq,
		Vphsubsw,
		Vphsubw,
		Vphsubwd,
		Vpinsrb,
		Vpinsrd,
		Vpinsrq,
		Vpinsrw,
		Vplzcntd,
		Vplzcntq,
		Vpmacsdd,
		Vpmacsdqh,
		Vpmacsdql,
		Vpmacssdd,
		Vpmacssdqh,
		Vpmacssdql,
		Vpmacsswd,
		Vpmacssww,
		Vpmacswd,
		Vpmacsww,
		Vpmadcsswd,
		Vpmadcswd,
		Vpmadd52huq,
		Vpmadd52luq,
		Vpmaddubsw,
		Vpmaddwd,
		Vpmaskmovd,
		Vpmaskmovq,
		Vpmaxsb,
		Vpmaxsd,
		Vpmaxsq,
		Vpmaxsw,
		Vpmaxub,
		Vpmaxud,
		Vpmaxuq,
		Vpmaxuw,
		Vpminsb,
		Vpminsd,
		Vpminsq,
		Vpminsw,
		Vpminub,
		Vpminud,
		Vpminuq,
		Vpminuw,
		Vpmovb2m,
		Vpmovd2m,
		Vpmovdb,
		Vpmovdw,
		Vpmovm2b,
		Vpmovm2d,
		Vpmovm2q,
		Vpmovm2w,
		Vpmovmskb,
		Vpmovq2m,
		Vpmovqb,
		Vpmovqd,
		Vpmovqw,
		Vpmovsdb,
		Vpmovsdw,
		Vpmovsqb,
		Vpmovsqd,
		Vpmovsqw,
		Vpmovswb,
		Vpmovsxbd,
		Vpmovsxbq,
		Vpmovsxbw,
		Vpmovsxdq,
		Vpmovsxwd,
		Vpmovsxwq,
		Vpmovusdb,
		Vpmovusdw,
		Vpmovusqb,
		Vpmovusqd,
		Vpmovusqw,
		Vpmovuswb,
		Vpmovw2m,
		Vpmovwb,
		Vpmovzxbd,
		Vpmovzxbq,
		Vpmovzxbw,
		Vpmovzxdq,
		Vpmovzxwd,
		Vpmovzxwq,
		Vpmuldq,
		Vpmulhrsw,
		Vpmulhuw,
		Vpmulhw,
		Vpmulld,
		Vpmullq,
		Vpmullw,
		Vpmultishiftqb,
		Vpmuludq,
		Vpopcntb,
		Vpopcntd,
		Vpopcntq,
		Vpopcntw,
		Vpor,
		Vpord,
		Vporq,
		Vpperm,
		Vprold,
		Vprolq,
		Vprolvd,
		Vprolvq,
		Vprord,
		Vprorq,
		Vprorvd,
		Vprorvq,
		Vprotb,
		Vprotd,
		Vprotq,
		Vprotw,
		Vpsadbw,
		Vpscatterdd,
		Vpscatterdq,
		Vpscatterqd,
		Vpscatterqq,
		Vpshab,
		Vpshad,
		Vpshaq,
		Vpshaw,
		Vpshlb,
		Vpshld,
		Vpshldd,
		Vpshldq,
		Vpshldvd,
		Vpshldvq,
		Vpshldvw,
		Vpshldw,
		Vpshlq,
		Vpshlw,
		Vpshrdd,
		Vpshrdq,
		Vpshrdvd,
		Vpshrdvq,
		Vpshrdvw,
		Vpshrdw,
		Vpshufb,
		Vpshufbitqmb,
		Vpshufd,
		Vpshufhw,
		Vpshuflw,
		Vpsignb,
		Vpsignd,
		Vpsignw,
		Vpslld,
		Vpslldq,
		Vpsllq,
		Vpsllvd,
		Vpsllvq,
		Vpsllvw,
		Vpsllw,
		Vpsrad,
		Vpsraq,
		Vpsravd,
		Vpsravq,
		Vpsravw,
		Vpsraw,
		Vpsrld,
		Vpsrldq,
		Vpsrlq,
		Vpsrlvd,
		Vpsrlvq,
		Vpsrlvw,
		Vpsrlw,
		Vpsubb,
		Vpsubd,
		Vpsubq,
		Vpsubsb,
		Vpsubsw,
		Vpsubusb,
		Vpsubusw,
		Vpsubw,
		Vpternlogd,
		Vpternlogq,
		Vptest,
		Vptestmb,
		Vptestmd,
		Vptestmq,
		Vptestmw,
		Vptestnmb,
		Vptestnmd,
		Vptestnmq,
		Vptestnmw,
		Vpunpckhbw,
		Vpunpckhdq,
		Vpunpckhqdq,
		Vpunpckhwd,
		Vpunpcklbw,
		Vpunpckldq,
		Vpunpcklqdq,
		Vpunpcklwd,
		Vpxor,
		Vpxord,
		Vpxorq,
		Vrangepd,
		Vrangeps,
		Vrangesd,
		Vrangess,
		Vrcp14pd,
		Vrcp14ps,
		Vrcp14sd,
		Vrcp14ss,
		Vrcp28pd,
		Vrcp28ps,
		Vrcp28sd,
		Vrcp28ss,
		Vrcpps,
		Vrcpss,
		Vreducepd,
		Vreduceps,
		Vreducesd,
		Vreducess,
		Vrndscalepd,
		Vrndscaleps,
		Vrndscalesd,
		Vrndscaless,
		Vroundpd,
		Vroundps,
		Vroundsd,
		Vroundss,
		Vrsqrt14pd,
		Vrsqrt14ps,
		Vrsqrt14sd,
		Vrsqrt14ss,
		Vrsqrt28pd,
		Vrsqrt28ps,
		Vrsqrt28sd,
		Vrsqrt28ss,
		Vrsqrtps,
		Vrsqrtss,
		Vscalefpd,
		Vscalefps,
		Vscalefsd,
		Vscalefss,
		Vscatterdpd,
		Vscatterdps,
		Vscatterpf0dpd,
		Vscatterpf0dps,
		Vscatterpf0qpd,
		Vscatterpf0qps,
		Vscatterpf1dpd,
		Vscatterpf1dps,
		Vscatterpf1qpd,
		Vscatterpf1qps,
		Vscatterqpd,
		Vscatterqps,
		Vshuff32x4,
		Vshuff64x2,
		Vshufi32x4,
		Vshufi64x2,
		Vshufpd,
		Vshufps,
		Vsqrtpd,
		Vsqrtps,
		Vsqrtsd,
		Vsqrtss,
		Vstmxcsr,
		Vsubpd,
		Vsubps,
		Vsubsd,
		Vsubss,
		Vtestpd,
		Vtestps,
		Vucomisd,
		Vucomiss,
		Vunpckhpd,
		Vunpckhps,
		Vunpcklpd,
		Vunpcklps,
		Vxorpd,
		Vxorps,
		Vzeroall,
		Vzeroupper,
		Wait,
		Wbinvd,
		Wbnoinvd,
		Wrfsbase,
		Wrgsbase,
		Wrmsr,
		Wrpkru,
		Wrssd,
		Wrssq,
		Wrussd,
		Wrussq,
		Xabort,
		Xadd,
		Xbegin,
		Xbts,
		Xchg,
		Xcryptcbc,
		Xcryptcfb,
		Xcryptctr,
		Xcryptecb,
		Xcryptofb,
		Xend,
		Xgetbv,
		Xlatb,
		Xor,
		Xorpd,
		Xorps,
		Xrstor,
		Xrstor64,
		Xrstors,
		Xrstors64,
		Xsave,
		Xsave64,
		Xsavec,
		Xsavec64,
		Xsaveopt,
		Xsaveopt64,
		Xsaves,
		Xsaves64,
		Xsetbv,
		Xsha1,
		Xsha256,
		Xstore,
		Xtest,
		Rmpadjust,
		Rmpupdate,
		Psmash,
		Pvalidate,
		Serialize,
		Xsusldtrk,
		Xresldtrk,
		Invlpgb,
		Tlbsync,
		Vmgexit,
		Getsecq,
		Sysexitq,
		Ldtilecfg,
		Tilerelease,
		Sttilecfg,
		Tilezero,
		Tileloaddt1,
		Tilestored,
		Tileloadd,
		Tdpbf16ps,
		Tdpbuud,
		Tdpbusd,
		Tdpbsud,
		Tdpbssd,
		Sysretq,
		Fnstdw,
		Fnstsg,
		Rdshr,
		Wrshr,
		Smint,
		Dmint,
		Rdm,
		Svdc,
		Rsdc,
		Svldt,
		Rsldt,
		Svts,
		Rsts,
		Bb0_reset,
		Bb1_reset,
		Cpu_write,
		Cpu_read,
		Altinst,
		Paveb,
		Paddsiw,
		Pmagw,
		Pdistib,
		Psubsiw,
		Pmvzb,
		Pmvnzb,
		Pmvlzb,
		Pmvgezb,
		Pmulhriw,
		Pmachriw,
		Ftstp,
		Frint2,
		Frichop,
		Frinear,
		Undoc,
		Tdcall,
		Seamret,
		Seamops,
		Seamcall,
		Aesencwide128kl,
		Aesdecwide128kl,
		Aesencwide256kl,
		Aesdecwide256kl,
		Loadiwkey,
		Aesenc128kl,
		Aesdec128kl,
		Aesenc256kl,
		Aesdec256kl,
		Encodekey128,
		Encodekey256,
		Pushad,
		Popad,
		Pushfd,
		Pushfq,
		Popfd,
		Popfq,
		Iretd,
		Iretq,
		Int3,
		Uiret,
		Testui,
		Clui,
		Stui,
		Senduipi,
		Hreset,
		Ccs_hash,
		Ccs_encrypt,
	}
}
