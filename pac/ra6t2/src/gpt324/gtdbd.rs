#[doc = "Register `GTDBD` reader"]
pub struct R(crate::R<GTDBD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTDBD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTDBD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTDBD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTDBD` writer"]
pub struct W(crate::W<GTDBD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTDBD_SPEC>;
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
impl From<crate::W<GTDBD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTDBD_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Dead Time Buffer Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtdbd](index.html) module"]
pub struct GTDBD_SPEC;
impl crate::RegisterSpec for GTDBD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtdbd::R](R) reader structure"]
impl crate::Readable for GTDBD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtdbd::W](W) writer structure"]
impl crate::Writable for GTDBD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDBD to value 0xffff_ffff"]
impl crate::Resettable for GTDBD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
