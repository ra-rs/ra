#[doc = "Register `CCR2` reader"]
pub struct R(crate::R<CCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR2` writer"]
pub struct W(crate::W<CCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCP` reader - Base Clock Pulse"]
pub type BCP_R = crate::FieldReader<u8, BCP_A>;
#[doc = "Base Clock Pulse\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCP_A {
    #[doc = "0: 93 clock cycles (S = 93)"]
    _000 = 0,
    #[doc = "1: 128 clock cycles (S = 128)"]
    _001 = 1,
    #[doc = "2: 186 clock cycles (S = 186)"]
    _010 = 2,
    #[doc = "3: 512 clock cycles (S = 512)"]
    _011 = 3,
    #[doc = "4: 32 clock cycles (S = 32) (Initial value)"]
    _100 = 4,
    #[doc = "5: 64 clock cycles (S = 64)"]
    _101 = 5,
    #[doc = "6: 372 clock cycles (S = 372)"]
    _110 = 6,
    #[doc = "7: 256 clock cycles (S = 256)"]
    _111 = 7,
}
impl From<BCP_A> for u8 {
    #[inline(always)]
    fn from(variant: BCP_A) -> Self {
        variant as _
    }
}
impl BCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCP_A {
        match self.bits {
            0 => BCP_A::_000,
            1 => BCP_A::_001,
            2 => BCP_A::_010,
            3 => BCP_A::_011,
            4 => BCP_A::_100,
            5 => BCP_A::_101,
            6 => BCP_A::_110,
            7 => BCP_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == BCP_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == BCP_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == BCP_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == BCP_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == BCP_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == BCP_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == BCP_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == BCP_A::_111
    }
}
#[doc = "Field `BCP` writer - Base Clock Pulse"]
pub type BCP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR2_SPEC, u8, BCP_A, 3, O>;
impl<'a, const O: u8> BCP_W<'a, O> {
    #[doc = "93 clock cycles (S = 93)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(BCP_A::_000)
    }
    #[doc = "128 clock cycles (S = 128)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(BCP_A::_001)
    }
    #[doc = "186 clock cycles (S = 186)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(BCP_A::_010)
    }
    #[doc = "512 clock cycles (S = 512)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(BCP_A::_011)
    }
    #[doc = "32 clock cycles (S = 32) (Initial value)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(BCP_A::_100)
    }
    #[doc = "64 clock cycles (S = 64)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(BCP_A::_101)
    }
    #[doc = "372 clock cycles (S = 372)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(BCP_A::_110)
    }
    #[doc = "256 clock cycles (S = 256)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(BCP_A::_111)
    }
}
#[doc = "Field `BGDM` reader - Baud Rate Generator Double-Speed Mode Select"]
pub type BGDM_R = crate::BitReader<BGDM_A>;
#[doc = "Baud Rate Generator Double-Speed Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGDM_A {
    #[doc = "0: Baud rate generator outputs the clock with single frequency."]
    _0 = 0,
    #[doc = "1: Baud rate generator outputs the clock with doubled frequency."]
    _1 = 1,
}
impl From<BGDM_A> for bool {
    #[inline(always)]
    fn from(variant: BGDM_A) -> Self {
        variant as u8 != 0
    }
}
impl BGDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGDM_A {
        match self.bits {
            false => BGDM_A::_0,
            true => BGDM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGDM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGDM_A::_1
    }
}
#[doc = "Field `BGDM` writer - Baud Rate Generator Double-Speed Mode Select"]
pub type BGDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR2_SPEC, BGDM_A, O>;
impl<'a, const O: u8> BGDM_W<'a, O> {
    #[doc = "Baud rate generator outputs the clock with single frequency."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGDM_A::_0)
    }
    #[doc = "Baud rate generator outputs the clock with doubled frequency."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGDM_A::_1)
    }
}
#[doc = "Field `ABCS` reader - Asynchronous Mode Base Clock Select"]
pub type ABCS_R = crate::BitReader<ABCS_A>;
#[doc = "Asynchronous Mode Base Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABCS_A {
    #[doc = "0: Selects 16 base clock cycles for 1-bit period."]
    _0 = 0,
    #[doc = "1: Selects 8 base clock cycles for 1-bit period."]
    _1 = 1,
}
impl From<ABCS_A> for bool {
    #[inline(always)]
    fn from(variant: ABCS_A) -> Self {
        variant as u8 != 0
    }
}
impl ABCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABCS_A {
        match self.bits {
            false => ABCS_A::_0,
            true => ABCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABCS_A::_1
    }
}
#[doc = "Field `ABCS` writer - Asynchronous Mode Base Clock Select"]
pub type ABCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR2_SPEC, ABCS_A, O>;
impl<'a, const O: u8> ABCS_W<'a, O> {
    #[doc = "Selects 16 base clock cycles for 1-bit period."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABCS_A::_0)
    }
    #[doc = "Selects 8 base clock cycles for 1-bit period."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABCS_A::_1)
    }
}
#[doc = "Field `ABCSE` reader - Asynchronous Mode Extended Base Clock Select"]
pub type ABCSE_R = crate::BitReader<ABCSE_A>;
#[doc = "Asynchronous Mode Extended Base Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABCSE_A {
    #[doc = "0: Clock cycles for 1-bit period is decided with combination be-tween CCR2.BGDM and CCR2.ABCS."]
    _0 = 0,
    #[doc = "1: Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
    _1 = 1,
}
impl From<ABCSE_A> for bool {
    #[inline(always)]
    fn from(variant: ABCSE_A) -> Self {
        variant as u8 != 0
    }
}
impl ABCSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABCSE_A {
        match self.bits {
            false => ABCSE_A::_0,
            true => ABCSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABCSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABCSE_A::_1
    }
}
#[doc = "Field `ABCSE` writer - Asynchronous Mode Extended Base Clock Select"]
pub type ABCSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR2_SPEC, ABCSE_A, O>;
impl<'a, const O: u8> ABCSE_W<'a, O> {
    #[doc = "Clock cycles for 1-bit period is decided with combination be-tween CCR2.BGDM and CCR2.ABCS."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABCSE_A::_0)
    }
    #[doc = "Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABCSE_A::_1)
    }
}
#[doc = "Field `BRR` reader - Bit rate setting"]
pub type BRR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRR` writer - Bit rate setting"]
pub type BRR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `BRME` reader - Bit Modulation Enable"]
pub type BRME_R = crate::BitReader<BRME_A>;
#[doc = "Bit Modulation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRME_A {
    #[doc = "0: Bit rate modulation function is disabled."]
    _0 = 0,
    #[doc = "1: Bit rate modulation function is enabled."]
    _1 = 1,
}
impl From<BRME_A> for bool {
    #[inline(always)]
    fn from(variant: BRME_A) -> Self {
        variant as u8 != 0
    }
}
impl BRME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRME_A {
        match self.bits {
            false => BRME_A::_0,
            true => BRME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRME_A::_1
    }
}
#[doc = "Field `BRME` writer - Bit Modulation Enable"]
pub type BRME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR2_SPEC, BRME_A, O>;
impl<'a, const O: u8> BRME_W<'a, O> {
    #[doc = "Bit rate modulation function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRME_A::_0)
    }
    #[doc = "Bit rate modulation function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRME_A::_1)
    }
}
#[doc = "Field `CKS` reader - Clock Select"]
pub type CKS_R = crate::FieldReader<u8, CKS_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKS_A {
    #[doc = "0: TCLK clock (n = 0)"]
    _00 = 0,
    #[doc = "1: TCLK/4 clock (n = 1)"]
    _01 = 1,
    #[doc = "2: TCLK/16 clock (n = 2)"]
    _10 = 2,
    #[doc = "3: TCLK/64 clock (n = 3)"]
    _11 = 3,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
impl CKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKS_A {
        match self.bits {
            0 => CKS_A::_00,
            1 => CKS_A::_01,
            2 => CKS_A::_10,
            3 => CKS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CKS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CKS_A::_11
    }
}
#[doc = "Field `CKS` writer - Clock Select"]
pub type CKS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR2_SPEC, u8, CKS_A, 2, O>;
impl<'a, const O: u8> CKS_W<'a, O> {
    #[doc = "TCLK clock (n = 0)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CKS_A::_00)
    }
    #[doc = "TCLK/4 clock (n = 1)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CKS_A::_01)
    }
    #[doc = "TCLK/16 clock (n = 2)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CKS_A::_10)
    }
    #[doc = "TCLK/64 clock (n = 3)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CKS_A::_11)
    }
}
#[doc = "Field `MDDR` reader - Modulation Duty Setting"]
pub type MDDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MDDR` writer - Modulation Duty Setting"]
pub type MDDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - Base Clock Pulse"]
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Baud Rate Generator Double-Speed Mode Select"]
    #[inline(always)]
    pub fn bgdm(&self) -> BGDM_R {
        BGDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Mode Base Clock Select"]
    #[inline(always)]
    pub fn abcs(&self) -> ABCS_R {
        ABCS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Asynchronous Mode Extended Base Clock Select"]
    #[inline(always)]
    pub fn abcse(&self) -> ABCSE_R {
        ABCSE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Bit rate setting"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Bit Modulation Enable"]
    #[inline(always)]
    pub fn brme(&self) -> BRME_R {
        BRME_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Clock Select"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Modulation Duty Setting"]
    #[inline(always)]
    pub fn mddr(&self) -> MDDR_R {
        MDDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Base Clock Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn bcp(&mut self) -> BCP_W<0> {
        BCP_W::new(self)
    }
    #[doc = "Bit 4 - Baud Rate Generator Double-Speed Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn bgdm(&mut self) -> BGDM_W<4> {
        BGDM_W::new(self)
    }
    #[doc = "Bit 5 - Asynchronous Mode Base Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn abcs(&mut self) -> ABCS_W<5> {
        ABCS_W::new(self)
    }
    #[doc = "Bit 6 - Asynchronous Mode Extended Base Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn abcse(&mut self) -> ABCSE_W<6> {
        ABCSE_W::new(self)
    }
    #[doc = "Bits 8:15 - Bit rate setting"]
    #[inline(always)]
    #[must_use]
    pub fn brr(&mut self) -> BRR_W<8> {
        BRR_W::new(self)
    }
    #[doc = "Bit 16 - Bit Modulation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brme(&mut self) -> BRME_W<16> {
        BRME_W::new(self)
    }
    #[doc = "Bits 20:21 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cks(&mut self) -> CKS_W<20> {
        CKS_W::new(self)
    }
    #[doc = "Bits 24:31 - Modulation Duty Setting"]
    #[inline(always)]
    #[must_use]
    pub fn mddr(&mut self) -> MDDR_W<24> {
        MDDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr2](index.html) module"]
pub struct CCR2_SPEC;
impl crate::RegisterSpec for CCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr2::R](R) reader structure"]
impl crate::Readable for CCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr2::W](W) writer structure"]
impl crate::Writable for CCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR2 to value 0xff00_ff04"]
impl crate::Resettable for CCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_ff04;
}
