#[doc = "Register `PITCH` writer"]
pub struct W(crate::W<PITCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PITCH_SPEC>;
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
impl From<crate::W<PITCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PITCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PITCH` writer - pitch of the framebuffer. A negative width can be used to render bottom-up instead of top-down"]
pub type PITCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PITCH_SPEC, u16, u16, 16, O>;
#[doc = "Field `SSD` writer - Spanstore delay"]
pub type SSD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PITCH_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - pitch of the framebuffer. A negative width can be used to render bottom-up instead of top-down"]
    #[inline(always)]
    #[must_use]
    pub fn pitch(&mut self) -> PITCH_W<0> {
        PITCH_W::new(self)
    }
    #[doc = "Bits 16:31 - Spanstore delay"]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SSD_W<16> {
        SSD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Framebuffer Pitch And Spanstore Delay Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pitch](index.html) module"]
pub struct PITCH_SPEC;
impl crate::RegisterSpec for PITCH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pitch::W](W) writer structure"]
impl crate::Writable for PITCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PITCH to value 0"]
impl crate::Resettable for PITCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
