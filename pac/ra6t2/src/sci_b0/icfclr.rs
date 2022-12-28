#[doc = "Register `ICFCLR` writer"]
pub struct W(crate::W<ICFCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICFCLR_SPEC>;
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
impl From<crate::W<ICFCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICFCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IICSTIFC` writer - IICSTIF clear bit"]
pub type IICSTIFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICFCLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 3 - IICSTIF clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn iicstifc(&mut self) -> IICSTIFC_W<3> {
        IICSTIFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Simple IIC Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfclr](index.html) module"]
pub struct ICFCLR_SPEC;
impl crate::RegisterSpec for ICFCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icfclr::W](W) writer structure"]
impl crate::Writable for ICFCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICFCLR to value 0"]
impl crate::Resettable for ICFCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
