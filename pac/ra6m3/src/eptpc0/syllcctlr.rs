#[doc = "Register `SYLLCCTLR` reader"]
pub struct R(crate::R<SYLLCCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYLLCCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYLLCCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYLLCCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYLLCCTLR` writer"]
pub struct W(crate::W<SYLLCCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYLLCCTLR_SPEC>;
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
impl From<crate::W<SYLLCCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYLLCCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTL` reader - LLC-CTL FieldThese bits specify the value used for the control field in the LLC sublayer when generating IEEE802.3 frames."]
pub type CTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTL` writer - LLC-CTL FieldThese bits specify the value used for the control field in the LLC sublayer when generating IEEE802.3 frames."]
pub type CTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYLLCCTLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LLC-CTL FieldThese bits specify the value used for the control field in the LLC sublayer when generating IEEE802.3 frames."]
    #[inline(always)]
    pub fn ctl(&self) -> CTL_R {
        CTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LLC-CTL FieldThese bits specify the value used for the control field in the LLC sublayer when generating IEEE802.3 frames."]
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CTL_W<0> {
        CTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYNFP LLC-CTL Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syllcctlr](index.html) module"]
pub struct SYLLCCTLR_SPEC;
impl crate::RegisterSpec for SYLLCCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syllcctlr::R](R) reader structure"]
impl crate::Readable for SYLLCCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syllcctlr::W](W) writer structure"]
impl crate::Writable for SYLLCCTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYLLCCTLR to value 0x03"]
impl crate::Resettable for SYLLCCTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
