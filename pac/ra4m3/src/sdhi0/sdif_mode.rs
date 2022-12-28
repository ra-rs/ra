#[doc = "Register `SDIF_MODE` reader"]
pub struct R(crate::R<SDIF_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIF_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIF_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIF_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIF_MODE` writer"]
pub struct W(crate::W<SDIF_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIF_MODE_SPEC>;
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
impl From<crate::W<SDIF_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIF_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOCHKCR` reader - CRC Check Mask"]
pub type NOCHKCR_R = crate::BitReader<NOCHKCR_A>;
#[doc = "CRC Check Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOCHKCR_A {
    #[doc = "0: Enable CRC check"]
    _0 = 0,
    #[doc = "1: Disable CRC Check (ignore CRC16 valued when reading and ignore CRC status value when writing)"]
    _1 = 1,
}
impl From<NOCHKCR_A> for bool {
    #[inline(always)]
    fn from(variant: NOCHKCR_A) -> Self {
        variant as u8 != 0
    }
}
impl NOCHKCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOCHKCR_A {
        match self.bits {
            false => NOCHKCR_A::_0,
            true => NOCHKCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NOCHKCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NOCHKCR_A::_1
    }
}
#[doc = "Field `NOCHKCR` writer - CRC Check Mask"]
pub type NOCHKCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIF_MODE_SPEC, NOCHKCR_A, O>;
impl<'a, const O: u8> NOCHKCR_W<'a, O> {
    #[doc = "Enable CRC check"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOCHKCR_A::_0)
    }
    #[doc = "Disable CRC Check (ignore CRC16 valued when reading and ignore CRC status value when writing)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOCHKCR_A::_1)
    }
}
impl R {
    #[doc = "Bit 8 - CRC Check Mask"]
    #[inline(always)]
    pub fn nochkcr(&self) -> NOCHKCR_R {
        NOCHKCR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - CRC Check Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nochkcr(&mut self) -> NOCHKCR_W<8> {
        NOCHKCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD Interface Mode Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdif_mode](index.html) module"]
pub struct SDIF_MODE_SPEC;
impl crate::RegisterSpec for SDIF_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdif_mode::R](R) reader structure"]
impl crate::Readable for SDIF_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdif_mode::W](W) writer structure"]
impl crate::Writable for SDIF_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIF_MODE to value 0"]
impl crate::Resettable for SDIF_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
