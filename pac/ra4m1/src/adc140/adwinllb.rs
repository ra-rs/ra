#[doc = "Register `ADWINLLB` reader"]
pub struct R(crate::R<ADWINLLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADWINLLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADWINLLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADWINLLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADWINLLB` writer"]
pub struct W(crate::W<ADWINLLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADWINLLB_SPEC>;
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
impl From<crate::W<ADWINLLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADWINLLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADWINLLB` reader - This register is used to compare A window function is used to set the lower level of the window B."]
pub type ADWINLLB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADWINLLB` writer - This register is used to compare A window function is used to set the lower level of the window B."]
pub type ADWINLLB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ADWINLLB_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register is used to compare A window function is used to set the lower level of the window B."]
    #[inline(always)]
    pub fn adwinllb(&self) -> ADWINLLB_R {
        ADWINLLB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to compare A window function is used to set the lower level of the window B."]
    #[inline(always)]
    #[must_use]
    pub fn adwinllb(&mut self) -> ADWINLLB_W<0> {
        ADWINLLB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window B Lower-Side Level Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adwinllb](index.html) module"]
pub struct ADWINLLB_SPEC;
impl crate::RegisterSpec for ADWINLLB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adwinllb::R](R) reader structure"]
impl crate::Readable for ADWINLLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adwinllb::W](W) writer structure"]
impl crate::Writable for ADWINLLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADWINLLB to value 0"]
impl crate::Resettable for ADWINLLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
