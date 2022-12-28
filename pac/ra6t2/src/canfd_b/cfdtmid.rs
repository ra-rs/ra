#[doc = "Register `CFDTMID%s` reader"]
pub struct R(crate::R<CFDTMID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDTMID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDTMID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDTMID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDTMID%s` writer"]
pub struct W(crate::W<CFDTMID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDTMID_SPEC>;
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
impl From<crate::W<CFDTMID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDTMID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMID` reader - TX Message Buffer ID Field"]
pub type TMID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TMID` writer - TX Message Buffer ID Field"]
pub type TMID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDTMID_SPEC, u32, u32, 29, O>;
#[doc = "Field `THLEN` reader - Tx History List Entry"]
pub type THLEN_R = crate::BitReader<THLEN_A>;
#[doc = "Tx History List Entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLEN_A {
    #[doc = "0: Entry not stored in THL after successful TX"]
    _0 = 0,
    #[doc = "1: Entry stored in THL after successful TX"]
    _1 = 1,
}
impl From<THLEN_A> for bool {
    #[inline(always)]
    fn from(variant: THLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl THLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THLEN_A {
        match self.bits {
            false => THLEN_A::_0,
            true => THLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THLEN_A::_1
    }
}
#[doc = "Field `THLEN` writer - Tx History List Entry"]
pub type THLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTMID_SPEC, THLEN_A, O>;
impl<'a, const O: u8> THLEN_W<'a, O> {
    #[doc = "Entry not stored in THL after successful TX"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THLEN_A::_0)
    }
    #[doc = "Entry stored in THL after successful TX"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THLEN_A::_1)
    }
}
#[doc = "Field `TMRTR` reader - TX Message Buffer RTR bit"]
pub type TMRTR_R = crate::BitReader<TMRTR_A>;
#[doc = "TX Message Buffer RTR bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMRTR_A {
    #[doc = "0: Data frame"]
    _0 = 0,
    #[doc = "1: Remote frame"]
    _1 = 1,
}
impl From<TMRTR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl TMRTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRTR_A {
        match self.bits {
            false => TMRTR_A::_0,
            true => TMRTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMRTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMRTR_A::_1
    }
}
#[doc = "Field `TMRTR` writer - TX Message Buffer RTR bit"]
pub type TMRTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTMID_SPEC, TMRTR_A, O>;
impl<'a, const O: u8> TMRTR_W<'a, O> {
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMRTR_A::_0)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMRTR_A::_1)
    }
}
#[doc = "Field `TMIDE` reader - TX Message Buffer IDE bit"]
pub type TMIDE_R = crate::BitReader<TMIDE_A>;
#[doc = "TX Message Buffer IDE bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMIDE_A {
    #[doc = "0: STD-ID is transmitted"]
    _0 = 0,
    #[doc = "1: EXT-ID is transmitted"]
    _1 = 1,
}
impl From<TMIDE_A> for bool {
    #[inline(always)]
    fn from(variant: TMIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl TMIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMIDE_A {
        match self.bits {
            false => TMIDE_A::_0,
            true => TMIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMIDE_A::_1
    }
}
#[doc = "Field `TMIDE` writer - TX Message Buffer IDE bit"]
pub type TMIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDTMID_SPEC, TMIDE_A, O>;
impl<'a, const O: u8> TMIDE_W<'a, O> {
    #[doc = "STD-ID is transmitted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMIDE_A::_0)
    }
    #[doc = "EXT-ID is transmitted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMIDE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:28 - TX Message Buffer ID Field"]
    #[inline(always)]
    pub fn tmid(&self) -> TMID_R {
        TMID_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Tx History List Entry"]
    #[inline(always)]
    pub fn thlen(&self) -> THLEN_R {
        THLEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TX Message Buffer RTR bit"]
    #[inline(always)]
    pub fn tmrtr(&self) -> TMRTR_R {
        TMRTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TX Message Buffer IDE bit"]
    #[inline(always)]
    pub fn tmide(&self) -> TMIDE_R {
        TMIDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - TX Message Buffer ID Field"]
    #[inline(always)]
    #[must_use]
    pub fn tmid(&mut self) -> TMID_W<0> {
        TMID_W::new(self)
    }
    #[doc = "Bit 29 - Tx History List Entry"]
    #[inline(always)]
    #[must_use]
    pub fn thlen(&mut self) -> THLEN_W<29> {
        THLEN_W::new(self)
    }
    #[doc = "Bit 30 - TX Message Buffer RTR bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmrtr(&mut self) -> TMRTR_W<30> {
        TMRTR_W::new(self)
    }
    #[doc = "Bit 31 - TX Message Buffer IDE bit"]
    #[inline(always)]
    #[must_use]
    pub fn tmide(&mut self) -> TMIDE_W<31> {
        TMIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Message Buffer ID Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdtmid](index.html) module"]
pub struct CFDTMID_SPEC;
impl crate::RegisterSpec for CFDTMID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdtmid::R](R) reader structure"]
impl crate::Readable for CFDTMID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdtmid::W](W) writer structure"]
impl crate::Writable for CFDTMID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDTMID%s to value 0"]
impl crate::Resettable for CFDTMID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
