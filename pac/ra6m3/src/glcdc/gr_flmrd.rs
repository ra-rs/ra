#[doc = "Register `GR%s_FLMRD` reader"]
pub struct R(crate::R<GR_FLMRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GR_FLMRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GR_FLMRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GR_FLMRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GR%s_FLMRD` writer"]
pub struct W(crate::W<GR_FLMRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GR_FLMRD_SPEC>;
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
impl From<crate::W<GR_FLMRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GR_FLMRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RENB` reader - Graphics data (frame buffer data) read enable."]
pub type RENB_R = crate::BitReader<RENB_A>;
#[doc = "Graphics data (frame buffer data) read enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RENB_A {
    #[doc = "1: Enables reading."]
    _1 = 1,
    #[doc = "0: Disables reading."]
    _0 = 0,
}
impl From<RENB_A> for bool {
    #[inline(always)]
    fn from(variant: RENB_A) -> Self {
        variant as u8 != 0
    }
}
impl RENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RENB_A {
        match self.bits {
            true => RENB_A::_1,
            false => RENB_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RENB_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RENB_A::_0
    }
}
#[doc = "Field `RENB` writer - Graphics data (frame buffer data) read enable."]
pub type RENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GR_FLMRD_SPEC, RENB_A, O>;
impl<'a, const O: u8> RENB_W<'a, O> {
    #[doc = "Enables reading."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RENB_A::_1)
    }
    #[doc = "Disables reading."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RENB_A::_0)
    }
}
impl R {
    #[doc = "Bit 0 - Graphics data (frame buffer data) read enable."]
    #[inline(always)]
    pub fn renb(&self) -> RENB_R {
        RENB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Graphics data (frame buffer data) read enable."]
    #[inline(always)]
    #[must_use]
    pub fn renb(&mut self) -> RENB_W<0> {
        RENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphics %s Frame Buffer Read Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gr_flmrd](index.html) module"]
pub struct GR_FLMRD_SPEC;
impl crate::RegisterSpec for GR_FLMRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gr_flmrd::R](R) reader structure"]
impl crate::Readable for GR_FLMRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gr_flmrd::W](W) writer structure"]
impl crate::Writable for GR_FLMRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GR%s_FLMRD to value 0"]
impl crate::Resettable for GR_FLMRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
