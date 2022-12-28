#[doc = "Register `SDMOD` reader"]
pub struct R(crate::R<SDMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMOD` writer"]
pub struct W(crate::W<SDMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMOD_SPEC>;
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
impl From<crate::W<SDMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR` reader - Mode Register SettingWriting to these bits: Mode register set command is issued."]
pub type MR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MR` writer - Mode Register SettingWriting to these bits: Mode register set command is issued."]
pub type MR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SDMOD_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Mode Register SettingWriting to these bits: Mode register set command is issued."]
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(self.bits & 0x7fff)
    }
}
impl W {
    #[doc = "Bits 0:14 - Mode Register SettingWriting to these bits: Mode register set command is issued."]
    #[inline(always)]
    #[must_use]
    pub fn mr(&mut self) -> MR_W<0> {
        MR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmod](index.html) module"]
pub struct SDMOD_SPEC;
impl crate::RegisterSpec for SDMOD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sdmod::R](R) reader structure"]
impl crate::Readable for SDMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmod::W](W) writer structure"]
impl crate::Writable for SDMOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMOD to value 0"]
impl crate::Resettable for SDMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
