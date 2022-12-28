#[doc = "Register `SARL%s` reader"]
pub struct R(crate::R<SARL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SARL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SARL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SARL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SARL%s` writer"]
pub struct W(crate::W<SARL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SARL_SPEC>;
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
impl From<crate::W<SARL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SARL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVA` reader - A slave address is set.7-Bit Address = SVA\\[7:1\\]
10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\]
}"]
pub type SVA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SVA` writer - A slave address is set.7-Bit Address = SVA\\[7:1\\]
10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\]
}"]
pub type SVA_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SARL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - A slave address is set.7-Bit Address = SVA\\[7:1\\]
10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\]
}"]
    #[inline(always)]
    pub fn sva(&self) -> SVA_R {
        SVA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - A slave address is set.7-Bit Address = SVA\\[7:1\\]
10-Bit Address = { SVA9,SVA8,SVA\\[7:0\\]
}"]
    #[inline(always)]
    #[must_use]
    pub fn sva(&mut self) -> SVA_W<0> {
        SVA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Address Register L%s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sarl](index.html) module"]
pub struct SARL_SPEC;
impl crate::RegisterSpec for SARL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sarl::R](R) reader structure"]
impl crate::Readable for SARL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sarl::W](W) writer structure"]
impl crate::Writable for SARL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SARL%s to value 0"]
impl crate::Resettable for SARL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
