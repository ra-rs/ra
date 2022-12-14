#[doc = "Register `ICBRH` reader"]
pub struct R(crate::R<ICBRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICBRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICBRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICBRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICBRH` writer"]
pub struct W(crate::W<ICBRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICBRH_SPEC>;
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
impl From<crate::W<ICBRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICBRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRH` reader - Bit Rate High-Level Period (High-level period of SCL clock)"]
pub type BRH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRH` writer - Bit Rate High-Level Period (High-level period of SCL clock)"]
pub type BRH_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ICBRH_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Bit Rate High-Level Period (High-level period of SCL clock)"]
    #[inline(always)]
    pub fn brh(&self) -> BRH_R {
        BRH_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit Rate High-Level Period (High-level period of SCL clock)"]
    #[inline(always)]
    #[must_use]
    pub fn brh(&mut self) -> BRH_W<0> {
        BRH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Bit Rate High-Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icbrh](index.html) module"]
pub struct ICBRH_SPEC;
impl crate::RegisterSpec for ICBRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icbrh::R](R) reader structure"]
impl crate::Readable for ICBRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icbrh::W](W) writer structure"]
impl crate::Writable for ICBRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICBRH to value 0xff"]
impl crate::Resettable for ICBRH_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
