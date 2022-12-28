#[doc = "Register `L%sXADD` writer"]
pub struct W(crate::W<LXADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LXADD_SPEC>;
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
impl From<crate::W<LXADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LXADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LXADD` writer - X-axis increment"]
pub type LXADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LXADD_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - X-axis increment"]
    #[inline(always)]
    #[must_use]
    pub fn lxadd(&mut self) -> LXADD_W<0> {
        LXADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Limiter %s X-Axis Increment Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lxadd](index.html) module"]
pub struct LXADD_SPEC;
impl crate::RegisterSpec for LXADD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lxadd::W](W) writer structure"]
impl crate::Writable for LXADD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L%sXADD to value 0"]
impl crate::Resettable for LXADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
