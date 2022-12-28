#[doc = "Register `DACR` reader"]
pub struct R(crate::R<DACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACR` writer"]
pub struct W(crate::W<DACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACR_SPEC>;
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
impl From<crate::W<DACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAOE0` reader - D/A Output Enable 0"]
pub type DAOE0_R = crate::BitReader<DAOE0_A>;
#[doc = "D/A Output Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAOE0_A {
    #[doc = "0: Analog output of channel 0 (DA0) is disabled."]
    _0 = 0,
    #[doc = "1: D/A conversion of channel 0 is enabled. Analog output of channel 0 (DA0) is enabled."]
    _1 = 1,
}
impl From<DAOE0_A> for bool {
    #[inline(always)]
    fn from(variant: DAOE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DAOE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAOE0_A {
        match self.bits {
            false => DAOE0_A::_0,
            true => DAOE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAOE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAOE0_A::_1
    }
}
#[doc = "Field `DAOE0` writer - D/A Output Enable 0"]
pub type DAOE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DACR_SPEC, DAOE0_A, O>;
impl<'a, const O: u8> DAOE0_W<'a, O> {
    #[doc = "Analog output of channel 0 (DA0) is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAOE0_A::_0)
    }
    #[doc = "D/A conversion of channel 0 is enabled. Analog output of channel 0 (DA0) is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAOE0_A::_1)
    }
}
impl R {
    #[doc = "Bit 6 - D/A Output Enable 0"]
    #[inline(always)]
    pub fn daoe0(&self) -> DAOE0_R {
        DAOE0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - D/A Output Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn daoe0(&mut self) -> DAOE0_W<6> {
        DAOE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "D/A Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacr](index.html) module"]
pub struct DACR_SPEC;
impl crate::RegisterSpec for DACR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dacr::R](R) reader structure"]
impl crate::Readable for DACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacr::W](W) writer structure"]
impl crate::Writable for DACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACR to value 0x1f"]
impl crate::Resettable for DACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
