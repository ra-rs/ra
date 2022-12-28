#[doc = "Register `CFDCFID` reader"]
pub struct R(crate::R<CFDCFID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCFID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDCFID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDCFID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDCFID` writer"]
pub struct W(crate::W<CFDCFID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCFID_SPEC>;
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
impl From<crate::W<CFDCFID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDCFID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFID` reader - Common FIFO Buffer ID Field"]
pub type CFID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CFID` writer - Common FIFO Buffer ID Field"]
pub type CFID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDCFID_SPEC, u32, u32, 29, O>;
#[doc = "Field `THLEN` reader - THL Entry enable"]
pub type THLEN_R = crate::BitReader<THLEN_A>;
#[doc = "THL Entry enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THLEN_A {
    #[doc = "0: Entry will not be stored in THL after successful TX."]
    _0 = 0,
    #[doc = "1: Entry will be stored in THL after successful TX."]
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
#[doc = "Field `THLEN` writer - THL Entry enable"]
pub type THLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFID_SPEC, THLEN_A, O>;
impl<'a, const O: u8> THLEN_W<'a, O> {
    #[doc = "Entry will not be stored in THL after successful TX."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(THLEN_A::_0)
    }
    #[doc = "Entry will be stored in THL after successful TX."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(THLEN_A::_1)
    }
}
#[doc = "Field `CFRTR` reader - Common FIFO Buffer RTR Bit"]
pub type CFRTR_R = crate::BitReader<CFRTR_A>;
#[doc = "Common FIFO Buffer RTR Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFRTR_A {
    #[doc = "0: Data Frame"]
    _0 = 0,
    #[doc = "1: Remote Frame"]
    _1 = 1,
}
impl From<CFRTR_A> for bool {
    #[inline(always)]
    fn from(variant: CFRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl CFRTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFRTR_A {
        match self.bits {
            false => CFRTR_A::_0,
            true => CFRTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFRTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFRTR_A::_1
    }
}
#[doc = "Field `CFRTR` writer - Common FIFO Buffer RTR Bit"]
pub type CFRTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFID_SPEC, CFRTR_A, O>;
impl<'a, const O: u8> CFRTR_W<'a, O> {
    #[doc = "Data Frame"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFRTR_A::_0)
    }
    #[doc = "Remote Frame"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFRTR_A::_1)
    }
}
#[doc = "Field `CFIDE` reader - Common FIFO Buffer IDE Bit"]
pub type CFIDE_R = crate::BitReader<CFIDE_A>;
#[doc = "Common FIFO Buffer IDE Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFIDE_A {
    #[doc = "0: STD-ID will be transmitted or has been received"]
    _0 = 0,
    #[doc = "1: EXT-ID will be transmitted or has been received"]
    _1 = 1,
}
impl From<CFIDE_A> for bool {
    #[inline(always)]
    fn from(variant: CFIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFIDE_A {
        match self.bits {
            false => CFIDE_A::_0,
            true => CFIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFIDE_A::_1
    }
}
#[doc = "Field `CFIDE` writer - Common FIFO Buffer IDE Bit"]
pub type CFIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFID_SPEC, CFIDE_A, O>;
impl<'a, const O: u8> CFIDE_W<'a, O> {
    #[doc = "STD-ID will be transmitted or has been received"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFIDE_A::_0)
    }
    #[doc = "EXT-ID will be transmitted or has been received"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFIDE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:28 - Common FIFO Buffer ID Field"]
    #[inline(always)]
    pub fn cfid(&self) -> CFID_R {
        CFID_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - THL Entry enable"]
    #[inline(always)]
    pub fn thlen(&self) -> THLEN_R {
        THLEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Common FIFO Buffer RTR Bit"]
    #[inline(always)]
    pub fn cfrtr(&self) -> CFRTR_R {
        CFRTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Common FIFO Buffer IDE Bit"]
    #[inline(always)]
    pub fn cfide(&self) -> CFIDE_R {
        CFIDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Common FIFO Buffer ID Field"]
    #[inline(always)]
    #[must_use]
    pub fn cfid(&mut self) -> CFID_W<0> {
        CFID_W::new(self)
    }
    #[doc = "Bit 29 - THL Entry enable"]
    #[inline(always)]
    #[must_use]
    pub fn thlen(&mut self) -> THLEN_W<29> {
        THLEN_W::new(self)
    }
    #[doc = "Bit 30 - Common FIFO Buffer RTR Bit"]
    #[inline(always)]
    #[must_use]
    pub fn cfrtr(&mut self) -> CFRTR_W<30> {
        CFRTR_W::new(self)
    }
    #[doc = "Bit 31 - Common FIFO Buffer IDE Bit"]
    #[inline(always)]
    #[must_use]
    pub fn cfide(&mut self) -> CFIDE_W<31> {
        CFIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common FIFO Access ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdcfid](index.html) module"]
pub struct CFDCFID_SPEC;
impl crate::RegisterSpec for CFDCFID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdcfid::R](R) reader structure"]
impl crate::Readable for CFDCFID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdcfid::W](W) writer structure"]
impl crate::Writable for CFDCFID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDCFID to value 0"]
impl crate::Resettable for CFDCFID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
