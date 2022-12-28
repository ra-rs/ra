#[doc = "Register `MDTR` reader"]
pub struct R(crate::R<MDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDTR` writer"]
pub struct W(crate::W<MDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDTR_SPEC>;
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
impl From<crate::W<MDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DV0DEL` reader - Device 0 delay setting"]
pub type DV0DEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DV0DEL` writer - Device 0 delay setting"]
pub type DV0DEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSERAM` reader - OM_DQS enable counter"]
pub type DQSERAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSERAM` writer - OM_DQS enable counter"]
pub type DQSERAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DQSESOPI` reader - OM_DQS enable counter"]
pub type DQSESOPI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSESOPI` writer - OM_DQS enable counter"]
pub type DQSESOPI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DV1DEL` reader - Device 1 delay setting"]
pub type DV1DEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DV1DEL` writer - Device 1 delay setting"]
pub type DV1DEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DQSEDOPI` reader - OM_DQS enable counter"]
pub type DQSEDOPI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DQSEDOPI` writer - OM_DQS enable counter"]
pub type DQSEDOPI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDTR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Device 0 delay setting"]
    #[inline(always)]
    pub fn dv0del(&self) -> DV0DEL_R {
        DV0DEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - OM_DQS enable counter"]
    #[inline(always)]
    pub fn dqseram(&self) -> DQSERAM_R {
        DQSERAM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - OM_DQS enable counter"]
    #[inline(always)]
    pub fn dqsesopi(&self) -> DQSESOPI_R {
        DQSESOPI_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Device 1 delay setting"]
    #[inline(always)]
    pub fn dv1del(&self) -> DV1DEL_R {
        DV1DEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - OM_DQS enable counter"]
    #[inline(always)]
    pub fn dqsedopi(&self) -> DQSEDOPI_R {
        DQSEDOPI_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Device 0 delay setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv0del(&mut self) -> DV0DEL_W<0> {
        DV0DEL_W::new(self)
    }
    #[doc = "Bits 8:11 - OM_DQS enable counter"]
    #[inline(always)]
    #[must_use]
    pub fn dqseram(&mut self) -> DQSERAM_W<8> {
        DQSERAM_W::new(self)
    }
    #[doc = "Bits 12:15 - OM_DQS enable counter"]
    #[inline(always)]
    #[must_use]
    pub fn dqsesopi(&mut self) -> DQSESOPI_W<12> {
        DQSESOPI_W::new(self)
    }
    #[doc = "Bits 16:23 - Device 1 delay setting"]
    #[inline(always)]
    #[must_use]
    pub fn dv1del(&mut self) -> DV1DEL_W<16> {
        DV1DEL_W::new(self)
    }
    #[doc = "Bits 24:27 - OM_DQS enable counter"]
    #[inline(always)]
    #[must_use]
    pub fn dqsedopi(&mut self) -> DQSEDOPI_W<24> {
        DQSEDOPI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Delay Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdtr](index.html) module"]
pub struct MDTR_SPEC;
impl crate::RegisterSpec for MDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdtr::R](R) reader structure"]
impl crate::Readable for MDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdtr::W](W) writer structure"]
impl crate::Writable for MDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDTR to value 0x0600_9400"]
impl crate::Resettable for MDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600_9400;
}
