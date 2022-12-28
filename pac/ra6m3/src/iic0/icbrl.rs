#[doc = "Register `ICBRL` reader"]
pub struct R(crate::R<ICBRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICBRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICBRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICBRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICBRL` writer"]
pub struct W(crate::W<ICBRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICBRL_SPEC>;
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
impl From<crate::W<ICBRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICBRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRL` reader - Bit Rate Low-Level Period(Low-level period of SCL clock)"]
pub type BRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRL` writer - Bit Rate Low-Level Period(Low-level period of SCL clock)"]
pub type BRL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ICBRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Bit Rate Low-Level Period(Low-level period of SCL clock)"]
    #[inline(always)]
    pub fn brl(&self) -> BRL_R {
        BRL_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit Rate Low-Level Period(Low-level period of SCL clock)"]
    #[inline(always)]
    #[must_use]
    pub fn brl(&mut self) -> BRL_W<0> {
        BRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Bit Rate Low-Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icbrl](index.html) module"]
pub struct ICBRL_SPEC;
impl crate::RegisterSpec for ICBRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icbrl::R](R) reader structure"]
impl crate::Readable for ICBRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icbrl::W](W) writer structure"]
impl crate::Writable for ICBRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICBRL to value 0xff"]
impl crate::Resettable for ICBRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
