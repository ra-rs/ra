#[doc = "Register `AGTMR1` reader"]
pub struct R(crate::R<AGTMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AGTMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AGTMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AGTMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AGTMR1` writer"]
pub struct W(crate::W<AGTMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AGTMR1_SPEC>;
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
impl From<crate::W<AGTMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AGTMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMOD` reader - Operating Mode"]
pub type TMOD_R = crate::FieldReader<u8, TMOD_A>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMOD_A {
    #[doc = "0: Timer mode"]
    _000 = 0,
    #[doc = "1: Pulse output mode"]
    _001 = 1,
    #[doc = "2: Event counter mode"]
    _010 = 2,
    #[doc = "3: Pulse width measurement mode"]
    _011 = 3,
    #[doc = "4: Pulse period measurement mode"]
    _100 = 4,
}
impl From<TMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TMOD_A) -> Self {
        variant as _
    }
}
impl TMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMOD_A> {
        match self.bits {
            0 => Some(TMOD_A::_000),
            1 => Some(TMOD_A::_001),
            2 => Some(TMOD_A::_010),
            3 => Some(TMOD_A::_011),
            4 => Some(TMOD_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TMOD_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TMOD_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TMOD_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TMOD_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TMOD_A::_100
    }
}
#[doc = "Field `TMOD` writer - Operating Mode"]
pub type TMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, AGTMR1_SPEC, u8, TMOD_A, 3, O>;
impl<'a, const O: u8> TMOD_W<'a, O> {
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(TMOD_A::_000)
    }
    #[doc = "Pulse output mode"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(TMOD_A::_001)
    }
    #[doc = "Event counter mode"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(TMOD_A::_010)
    }
    #[doc = "Pulse width measurement mode"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(TMOD_A::_011)
    }
    #[doc = "Pulse period measurement mode"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(TMOD_A::_100)
    }
}
#[doc = "Field `TEDGPL` reader - Edge Polarity"]
pub type TEDGPL_R = crate::BitReader<TEDGPL_A>;
#[doc = "Edge Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEDGPL_A {
    #[doc = "0: Single-edge"]
    _0 = 0,
    #[doc = "1: Both-edge"]
    _1 = 1,
}
impl From<TEDGPL_A> for bool {
    #[inline(always)]
    fn from(variant: TEDGPL_A) -> Self {
        variant as u8 != 0
    }
}
impl TEDGPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEDGPL_A {
        match self.bits {
            false => TEDGPL_A::_0,
            true => TEDGPL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEDGPL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEDGPL_A::_1
    }
}
#[doc = "Field `TEDGPL` writer - Edge Polarity"]
pub type TEDGPL_W<'a, const O: u8> = crate::BitWriter<'a, u8, AGTMR1_SPEC, TEDGPL_A, O>;
impl<'a, const O: u8> TEDGPL_W<'a, O> {
    #[doc = "Single-edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEDGPL_A::_0)
    }
    #[doc = "Both-edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEDGPL_A::_1)
    }
}
#[doc = "Field `TCK` reader - Count Source"]
pub type TCK_R = crate::FieldReader<u8, TCK_A>;
#[doc = "Count Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCK_A {
    #[doc = "0: PCLKB"]
    _000 = 0,
    #[doc = "1: PCLKB/8"]
    _001 = 1,
    #[doc = "3: PCLKB/2"]
    _011 = 3,
    #[doc = "4: Divided clock AGTLCLK specified by CKS\\[2:0\\]
bits in the AGTMR2 register"]
    _100 = 4,
    #[doc = "5: Underflow event signal from AGTn (n = 0, 2)"]
    _101 = 5,
    #[doc = "6: Divided clock AGTSCLK specified by CKS\\[2:0\\]
bits in the AGTMR2 register"]
    _110 = 6,
}
impl From<TCK_A> for u8 {
    #[inline(always)]
    fn from(variant: TCK_A) -> Self {
        variant as _
    }
}
impl TCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCK_A> {
        match self.bits {
            0 => Some(TCK_A::_000),
            1 => Some(TCK_A::_001),
            3 => Some(TCK_A::_011),
            4 => Some(TCK_A::_100),
            5 => Some(TCK_A::_101),
            6 => Some(TCK_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TCK_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TCK_A::_001
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TCK_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TCK_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TCK_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == TCK_A::_110
    }
}
#[doc = "Field `TCK` writer - Count Source"]
pub type TCK_W<'a, const O: u8> = crate::FieldWriter<'a, u8, AGTMR1_SPEC, u8, TCK_A, 3, O>;
impl<'a, const O: u8> TCK_W<'a, O> {
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(TCK_A::_000)
    }
    #[doc = "PCLKB/8"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(TCK_A::_001)
    }
    #[doc = "PCLKB/2"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(TCK_A::_011)
    }
    #[doc = "Divided clock AGTLCLK specified by CKS\\[2:0\\]
bits in the AGTMR2 register"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(TCK_A::_100)
    }
    #[doc = "Underflow event signal from AGTn (n = 0, 2)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(TCK_A::_101)
    }
    #[doc = "Divided clock AGTSCLK specified by CKS\\[2:0\\]
bits in the AGTMR2 register"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(TCK_A::_110)
    }
}
impl R {
    #[doc = "Bits 0:2 - Operating Mode"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Edge Polarity"]
    #[inline(always)]
    pub fn tedgpl(&self) -> TEDGPL_R {
        TEDGPL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Count Source"]
    #[inline(always)]
    pub fn tck(&self) -> TCK_R {
        TCK_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmod(&mut self) -> TMOD_W<0> {
        TMOD_W::new(self)
    }
    #[doc = "Bit 3 - Edge Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tedgpl(&mut self) -> TEDGPL_W<3> {
        TEDGPL_W::new(self)
    }
    #[doc = "Bits 4:6 - Count Source"]
    #[inline(always)]
    #[must_use]
    pub fn tck(&mut self) -> TCK_W<4> {
        TCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AGT Mode Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [agtmr1](index.html) module"]
pub struct AGTMR1_SPEC;
impl crate::RegisterSpec for AGTMR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [agtmr1::R](R) reader structure"]
impl crate::Readable for AGTMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [agtmr1::W](W) writer structure"]
impl crate::Writable for AGTMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AGTMR1 to value 0"]
impl crate::Resettable for AGTMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
