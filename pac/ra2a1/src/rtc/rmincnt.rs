#[doc = "Register `RMINCNT` reader"]
pub struct R(crate::R<RMINCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMINCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMINCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMINCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMINCNT` writer"]
pub struct W(crate::W<RMINCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMINCNT_SPEC>;
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
impl From<crate::W<RMINCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMINCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN1` reader - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
pub type MIN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN1` writer - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
pub type MIN1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RMINCNT_SPEC, u8, u8, 4, O>;
#[doc = "Field `MIN10` reader - 10-Minute Count Counts from 0 to 5 for 60-minute counting."]
pub type MIN10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN10` writer - 10-Minute Count Counts from 0 to 5 for 60-minute counting."]
pub type MIN10_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RMINCNT_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3 - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn min1(&self) -> MIN1_R {
        MIN1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Minute Count Counts from 0 to 5 for 60-minute counting."]
    #[inline(always)]
    pub fn min10(&self) -> MIN10_R {
        MIN10_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Minute Count Counts from 0 to 9 every minute. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    #[must_use]
    pub fn min1(&mut self) -> MIN1_W<0> {
        MIN1_W::new(self)
    }
    #[doc = "Bits 4:6 - 10-Minute Count Counts from 0 to 5 for 60-minute counting."]
    #[inline(always)]
    #[must_use]
    pub fn min10(&mut self) -> MIN10_W<4> {
        MIN10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Minute Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmincnt](index.html) module"]
pub struct RMINCNT_SPEC;
impl crate::RegisterSpec for RMINCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rmincnt::R](R) reader structure"]
impl crate::Readable for RMINCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmincnt::W](W) writer structure"]
impl crate::Writable for RMINCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMINCNT to value 0"]
impl crate::Resettable for RMINCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
