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
#[doc = "Field `SVA0` reader - 10-bit Address LSB"]
pub type SVA0_R = crate::BitReader<bool>;
#[doc = "Field `SVA0` writer - 10-bit Address LSB"]
pub type SVA0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SARL_SPEC, bool, O>;
#[doc = "Field `SVA` reader - 7-bit Address/10-bit Address Lower Bits"]
pub type SVA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SVA` writer - 7-bit Address/10-bit Address Lower Bits"]
pub type SVA_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SARL_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - 10-bit Address LSB"]
    #[inline(always)]
    pub fn sva0(&self) -> SVA0_R {
        SVA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7-bit Address/10-bit Address Lower Bits"]
    #[inline(always)]
    pub fn sva(&self) -> SVA_R {
        SVA_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - 10-bit Address LSB"]
    #[inline(always)]
    #[must_use]
    pub fn sva0(&mut self) -> SVA0_W<0> {
        SVA0_W::new(self)
    }
    #[doc = "Bits 1:7 - 7-bit Address/10-bit Address Lower Bits"]
    #[inline(always)]
    #[must_use]
    pub fn sva(&mut self) -> SVA_W<1> {
        SVA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Address Register Ly\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sarl](index.html) module"]
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
