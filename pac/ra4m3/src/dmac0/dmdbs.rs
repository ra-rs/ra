#[doc = "Register `DMDBS` reader"]
pub struct R(crate::R<DMDBS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMDBS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMDBS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMDBS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMDBS` writer"]
pub struct W(crate::W<DMDBS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMDBS_SPEC>;
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
impl From<crate::W<DMDBS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMDBS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMDBSL` reader - Functions as data transfer counter in repeat-block transfer mode"]
pub type DMDBSL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMDBSL` writer - Functions as data transfer counter in repeat-block transfer mode"]
pub type DMDBSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMDBS_SPEC, u16, u16, 16, O>;
#[doc = "Field `DMDBSH` reader - Specifies the repeat-area size in repeat-block transfer mode"]
pub type DMDBSH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMDBSH` writer - Specifies the repeat-area size in repeat-block transfer mode"]
pub type DMDBSH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMDBS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Functions as data transfer counter in repeat-block transfer mode"]
    #[inline(always)]
    pub fn dmdbsl(&self) -> DMDBSL_R {
        DMDBSL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specifies the repeat-area size in repeat-block transfer mode"]
    #[inline(always)]
    pub fn dmdbsh(&self) -> DMDBSH_R {
        DMDBSH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Functions as data transfer counter in repeat-block transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmdbsl(&mut self) -> DMDBSL_W<0> {
        DMDBSL_W::new(self)
    }
    #[doc = "Bits 16:31 - Specifies the repeat-area size in repeat-block transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmdbsh(&mut self) -> DMDBSH_W<16> {
        DMDBSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Destination Buffer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmdbs](index.html) module"]
pub struct DMDBS_SPEC;
impl crate::RegisterSpec for DMDBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmdbs::R](R) reader structure"]
impl crate::Readable for DMDBS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmdbs::W](W) writer structure"]
impl crate::Writable for DMDBS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMDBS to value 0"]
impl crate::Resettable for DMDBS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
