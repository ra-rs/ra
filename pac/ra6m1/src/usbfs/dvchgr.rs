#[doc = "Register `DVCHGR` reader"]
pub struct R(crate::R<DVCHGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVCHGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVCHGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVCHGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVCHGR` writer"]
pub struct W(crate::W<DVCHGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVCHGR_SPEC>;
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
impl From<crate::W<DVCHGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVCHGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVCHG` reader - Device State Change"]
pub type DVCHG_R = crate::BitReader<DVCHG_A>;
#[doc = "Device State Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVCHG_A {
    #[doc = "0: Disables the writing to the USBADDR.STSRECOV\\[3:0\\]
bits and USBADDR.USBADDR\\[6:0\\]."]
    _0 = 0,
    #[doc = "1: Enables the writing to the USBADDR.STSRECOV\\[3:0\\]
bits and USBADDR.USBADDR\\[6:0\\]."]
    _1 = 1,
}
impl From<DVCHG_A> for bool {
    #[inline(always)]
    fn from(variant: DVCHG_A) -> Self {
        variant as u8 != 0
    }
}
impl DVCHG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVCHG_A {
        match self.bits {
            false => DVCHG_A::_0,
            true => DVCHG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVCHG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVCHG_A::_1
    }
}
#[doc = "Field `DVCHG` writer - Device State Change"]
pub type DVCHG_W<'a, const O: u8> = crate::BitWriter<'a, u16, DVCHGR_SPEC, DVCHG_A, O>;
impl<'a, const O: u8> DVCHG_W<'a, O> {
    #[doc = "Disables the writing to the USBADDR.STSRECOV\\[3:0\\]
bits and USBADDR.USBADDR\\[6:0\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVCHG_A::_0)
    }
    #[doc = "Enables the writing to the USBADDR.STSRECOV\\[3:0\\]
bits and USBADDR.USBADDR\\[6:0\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVCHG_A::_1)
    }
}
impl R {
    #[doc = "Bit 15 - Device State Change"]
    #[inline(always)]
    pub fn dvchg(&self) -> DVCHG_R {
        DVCHG_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Device State Change"]
    #[inline(always)]
    #[must_use]
    pub fn dvchg(&mut self) -> DVCHG_W<15> {
        DVCHG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device State Change Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvchgr](index.html) module"]
pub struct DVCHGR_SPEC;
impl crate::RegisterSpec for DVCHGR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dvchgr::R](R) reader structure"]
impl crate::Readable for DVCHGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvchgr::W](W) writer structure"]
impl crate::Writable for DVCHGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DVCHGR to value 0"]
impl crate::Resettable for DVCHGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
