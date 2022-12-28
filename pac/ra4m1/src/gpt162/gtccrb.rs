#[doc = "Register `GTCCRB` reader"]
pub struct R(crate::R<GTCCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCCRB` writer"]
pub struct W(crate::W<GTCCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCCRB_SPEC>;
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
impl From<crate::W<GTCCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTCCRB` reader - Compare Capture Register B"]
pub type GTCCRB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GTCCRB` writer - Compare Capture Register B"]
pub type GTCCRB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTCCRB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Compare Capture Register B"]
    #[inline(always)]
    pub fn gtccrb(&self) -> GTCCRB_R {
        GTCCRB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Capture Register B"]
    #[inline(always)]
    #[must_use]
    pub fn gtccrb(&mut self) -> GTCCRB_W<0> {
        GTCCRB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Compare Capture Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtccrb](index.html) module"]
pub struct GTCCRB_SPEC;
impl crate::RegisterSpec for GTCCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtccrb::R](R) reader structure"]
impl crate::Readable for GTCCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtccrb::W](W) writer structure"]
impl crate::Writable for GTCCRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTCCRB to value 0xffff"]
impl crate::Resettable for GTCCRB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
