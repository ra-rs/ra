#[doc = "Register `DMSBS` reader"]
pub struct R(crate::R<DMSBS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMSBS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMSBS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMSBS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMSBS` writer"]
pub struct W(crate::W<DMSBS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMSBS_SPEC>;
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
impl From<crate::W<DMSBS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMSBS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMSBSL` reader - Functions as data transfer counter in repeat-block transfer mode"]
pub type DMSBSL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMSBSL` writer - Functions as data transfer counter in repeat-block transfer mode"]
pub type DMSBSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMSBS_SPEC, u16, u16, 16, O>;
#[doc = "Field `DMSBSH` reader - Specifies the repeat-area size in repeat-block transfer mode"]
pub type DMSBSH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMSBSH` writer - Specifies the repeat-area size in repeat-block transfer mode"]
pub type DMSBSH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMSBS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Functions as data transfer counter in repeat-block transfer mode"]
    #[inline(always)]
    pub fn dmsbsl(&self) -> DMSBSL_R {
        DMSBSL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specifies the repeat-area size in repeat-block transfer mode"]
    #[inline(always)]
    pub fn dmsbsh(&self) -> DMSBSH_R {
        DMSBSH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Functions as data transfer counter in repeat-block transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmsbsl(&mut self) -> DMSBSL_W<0> {
        DMSBSL_W::new(self)
    }
    #[doc = "Bits 16:31 - Specifies the repeat-area size in repeat-block transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmsbsh(&mut self) -> DMSBSH_W<16> {
        DMSBSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Source Buffer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmsbs](index.html) module"]
pub struct DMSBS_SPEC;
impl crate::RegisterSpec for DMSBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmsbs::R](R) reader structure"]
impl crate::Readable for DMSBS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmsbs::W](W) writer structure"]
impl crate::Writable for DMSBS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMSBS to value 0"]
impl crate::Resettable for DMSBS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
