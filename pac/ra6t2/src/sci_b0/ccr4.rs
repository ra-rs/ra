#[doc = "Register `CCR4` reader"]
pub struct R(crate::R<CCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR4` writer"]
pub struct W(crate::W<CCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR4_SPEC>;
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
impl From<crate::W<CCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPD` reader - Compare Match Data"]
pub type CMPD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMPD` writer - Compare Match Data"]
pub type CMPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR4_SPEC, u16, u16, 9, O>;
#[doc = "Field `ASEN` reader - Adjust receive sampling timing enable"]
pub type ASEN_R = crate::BitReader<ASEN_A>;
#[doc = "Adjust receive sampling timing enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASEN_A {
    #[doc = "0: Adjust sampling timing disable."]
    _0 = 0,
    #[doc = "1: Adjust sampling timing enable."]
    _1 = 1,
}
impl From<ASEN_A> for bool {
    #[inline(always)]
    fn from(variant: ASEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASEN_A {
        match self.bits {
            false => ASEN_A::_0,
            true => ASEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASEN_A::_1
    }
}
#[doc = "Field `ASEN` writer - Adjust receive sampling timing enable"]
pub type ASEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, ASEN_A, O>;
impl<'a, const O: u8> ASEN_W<'a, O> {
    #[doc = "Adjust sampling timing disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASEN_A::_0)
    }
    #[doc = "Adjust sampling timing enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASEN_A::_1)
    }
}
#[doc = "Field `ATEN` reader - Adjust transmit timing enable"]
pub type ATEN_R = crate::BitReader<ATEN_A>;
#[doc = "Adjust transmit timing enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATEN_A {
    #[doc = "0: Adjust transmit timing disable."]
    _0 = 0,
    #[doc = "1: Adjust transmit timing enable."]
    _1 = 1,
}
impl From<ATEN_A> for bool {
    #[inline(always)]
    fn from(variant: ATEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATEN_A {
        match self.bits {
            false => ATEN_A::_0,
            true => ATEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATEN_A::_1
    }
}
#[doc = "Field `ATEN` writer - Adjust transmit timing enable"]
pub type ATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, ATEN_A, O>;
impl<'a, const O: u8> ATEN_W<'a, O> {
    #[doc = "Adjust transmit timing disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATEN_A::_0)
    }
    #[doc = "Adjust transmit timing enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATEN_A::_1)
    }
}
#[doc = "Field `AST` reader - Adjustment value for receive Sampling Timing"]
pub type AST_R = crate::FieldReader<u8, AST_A>;
#[doc = "Adjustment value for receive Sampling Timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AST_A {
    #[doc = "0: 1TCLK delay"]
    _000 = 0,
    #[doc = "1: 2TCLK delay"]
    _001 = 1,
    #[doc = "2: 3TCLK delay"]
    _010 = 2,
    #[doc = "3: 4TCLK delay"]
    _011 = 3,
}
impl From<AST_A> for u8 {
    #[inline(always)]
    fn from(variant: AST_A) -> Self {
        variant as _
    }
}
impl AST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AST_A> {
        match self.bits {
            0 => Some(AST_A::_000),
            1 => Some(AST_A::_001),
            2 => Some(AST_A::_010),
            3 => Some(AST_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == AST_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == AST_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == AST_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == AST_A::_011
    }
}
#[doc = "Field `AST` writer - Adjustment value for receive Sampling Timing"]
pub type AST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR4_SPEC, u8, AST_A, 3, O>;
impl<'a, const O: u8> AST_W<'a, O> {
    #[doc = "1TCLK delay"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(AST_A::_000)
    }
    #[doc = "2TCLK delay"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(AST_A::_001)
    }
    #[doc = "3TCLK delay"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(AST_A::_010)
    }
    #[doc = "4TCLK delay"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(AST_A::_011)
    }
}
#[doc = "Field `AJD` reader - Adjustment Direction for receive sampling timing"]
pub type AJD_R = crate::BitReader<AJD_A>;
#[doc = "Adjustment Direction for receive sampling timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AJD_A {
    #[doc = "0: The sampling timing is adjusted backward to the middle of bit."]
    _0 = 0,
    #[doc = "1: The sampling timing is adjusted forward to the middle of bit."]
    _1 = 1,
}
impl From<AJD_A> for bool {
    #[inline(always)]
    fn from(variant: AJD_A) -> Self {
        variant as u8 != 0
    }
}
impl AJD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AJD_A {
        match self.bits {
            false => AJD_A::_0,
            true => AJD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AJD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AJD_A::_1
    }
}
#[doc = "Field `AJD` writer - Adjustment Direction for receive sampling timing"]
pub type AJD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, AJD_A, O>;
impl<'a, const O: u8> AJD_W<'a, O> {
    #[doc = "The sampling timing is adjusted backward to the middle of bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AJD_A::_0)
    }
    #[doc = "The sampling timing is adjusted forward to the middle of bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AJD_A::_1)
    }
}
#[doc = "Field `ATT` reader - Adjustment value for Transmit timing"]
pub type ATT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATT` writer - Adjustment value for Transmit timing"]
pub type ATT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR4_SPEC, u8, u8, 3, O>;
#[doc = "Field `AET` reader - Adjustment edge for transmit timing"]
pub type AET_R = crate::BitReader<AET_A>;
#[doc = "Adjustment edge for transmit timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AET_A {
    #[doc = "0: When CCR1.TINV is 0, adjust the rising edge timing. When CCR1.TINV is 1, adjust the falling edge timing."]
    _0 = 0,
    #[doc = "1: When CCR1.TINV is 0, adjust the falling edge timing. When CCR1.TINV is 1, adjust the rising edge timing."]
    _1 = 1,
}
impl From<AET_A> for bool {
    #[inline(always)]
    fn from(variant: AET_A) -> Self {
        variant as u8 != 0
    }
}
impl AET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AET_A {
        match self.bits {
            false => AET_A::_0,
            true => AET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AET_A::_1
    }
}
#[doc = "Field `AET` writer - Adjustment edge for transmit timing"]
pub type AET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR4_SPEC, AET_A, O>;
impl<'a, const O: u8> AET_W<'a, O> {
    #[doc = "When CCR1.TINV is 0, adjust the rising edge timing. When CCR1.TINV is 1, adjust the falling edge timing."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AET_A::_0)
    }
    #[doc = "When CCR1.TINV is 0, adjust the falling edge timing. When CCR1.TINV is 1, adjust the rising edge timing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AET_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Compare Match Data"]
    #[inline(always)]
    pub fn cmpd(&self) -> CMPD_R {
        CMPD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Adjust receive sampling timing enable"]
    #[inline(always)]
    pub fn asen(&self) -> ASEN_R {
        ASEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Adjust transmit timing enable"]
    #[inline(always)]
    pub fn aten(&self) -> ATEN_R {
        ATEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Adjustment value for receive Sampling Timing"]
    #[inline(always)]
    pub fn ast(&self) -> AST_R {
        AST_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Adjustment Direction for receive sampling timing"]
    #[inline(always)]
    pub fn ajd(&self) -> AJD_R {
        AJD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Adjustment value for Transmit timing"]
    #[inline(always)]
    pub fn att(&self) -> ATT_R {
        ATT_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Adjustment edge for transmit timing"]
    #[inline(always)]
    pub fn aet(&self) -> AET_R {
        AET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Compare Match Data"]
    #[inline(always)]
    #[must_use]
    pub fn cmpd(&mut self) -> CMPD_W<0> {
        CMPD_W::new(self)
    }
    #[doc = "Bit 16 - Adjust receive sampling timing enable"]
    #[inline(always)]
    #[must_use]
    pub fn asen(&mut self) -> ASEN_W<16> {
        ASEN_W::new(self)
    }
    #[doc = "Bit 17 - Adjust transmit timing enable"]
    #[inline(always)]
    #[must_use]
    pub fn aten(&mut self) -> ATEN_W<17> {
        ATEN_W::new(self)
    }
    #[doc = "Bits 24:26 - Adjustment value for receive Sampling Timing"]
    #[inline(always)]
    #[must_use]
    pub fn ast(&mut self) -> AST_W<24> {
        AST_W::new(self)
    }
    #[doc = "Bit 27 - Adjustment Direction for receive sampling timing"]
    #[inline(always)]
    #[must_use]
    pub fn ajd(&mut self) -> AJD_W<27> {
        AJD_W::new(self)
    }
    #[doc = "Bits 28:30 - Adjustment value for Transmit timing"]
    #[inline(always)]
    #[must_use]
    pub fn att(&mut self) -> ATT_W<28> {
        ATT_W::new(self)
    }
    #[doc = "Bit 31 - Adjustment edge for transmit timing"]
    #[inline(always)]
    #[must_use]
    pub fn aet(&mut self) -> AET_W<31> {
        AET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr4](index.html) module"]
pub struct CCR4_SPEC;
impl crate::RegisterSpec for CCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr4::R](R) reader structure"]
impl crate::Readable for CCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr4::W](W) writer structure"]
impl crate::Writable for CCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR4 to value 0"]
impl crate::Resettable for CCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
