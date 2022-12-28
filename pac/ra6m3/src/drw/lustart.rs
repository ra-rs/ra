#[doc = "Register `LUSTART` writer"]
pub struct W(crate::W<LUSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUSTART_SPEC>;
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
impl From<crate::W<LUSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUSTART` writer - U limiter start value"]
pub type LUSTART_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUSTART_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - U limiter start value"]
    #[inline(always)]
    #[must_use]
    pub fn lustart(&mut self) -> LUSTART_W<0> {
        LUSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U Limiter Start Value Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lustart](index.html) module"]
pub struct LUSTART_SPEC;
impl crate::RegisterSpec for LUSTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lustart::W](W) writer structure"]
impl crate::Writable for LUSTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUSTART to value 0"]
impl crate::Resettable for LUSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
