#[doc = "Register `LUXADD` writer"]
pub struct W(crate::W<LUXADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUXADD_SPEC>;
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
impl From<crate::W<LUXADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUXADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUXADD` writer - U limiter x-axis increment"]
pub type LUXADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUXADD_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - U limiter x-axis increment"]
    #[inline(always)]
    #[must_use]
    pub fn luxadd(&mut self) -> LUXADD_W<0> {
        LUXADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U Limiter X-Axis Increment Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [luxadd](index.html) module"]
pub struct LUXADD_SPEC;
impl crate::RegisterSpec for LUXADD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [luxadd::W](W) writer structure"]
impl crate::Writable for LUXADD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUXADD to value 0"]
impl crate::Resettable for LUXADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
