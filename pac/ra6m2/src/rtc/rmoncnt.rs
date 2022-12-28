#[doc = "Register `RMONCNT` reader"]
pub struct R(crate::R<RMONCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMONCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMONCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMONCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMONCNT` writer"]
pub struct W(crate::W<RMONCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMONCNT_SPEC>;
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
impl From<crate::W<RMONCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMONCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MON1` reader - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
pub type MON1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON1` writer - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
pub type MON1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RMONCNT_SPEC, u8, u8, 4, O>;
#[doc = "Field `MON10` reader - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
pub type MON10_R = crate::BitReader<bool>;
#[doc = "Field `MON10` writer - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
pub type MON10_W<'a, const O: u8> = crate::BitWriter<'a, u8, RMONCNT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn mon1(&self) -> MON1_R {
        MON1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
    #[inline(always)]
    pub fn mon10(&self) -> MON10_R {
        MON10_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Month Count Counts from 0 to 9 once per month. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    #[must_use]
    pub fn mon1(&mut self) -> MON1_W<0> {
        MON1_W::new(self)
    }
    #[doc = "Bit 4 - 10-Month Count Counts from 0 to 1 once per carry from the ones place."]
    #[inline(always)]
    #[must_use]
    pub fn mon10(&mut self) -> MON10_W<4> {
        MON10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Month Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmoncnt](index.html) module"]
pub struct RMONCNT_SPEC;
impl crate::RegisterSpec for RMONCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rmoncnt::R](R) reader structure"]
impl crate::Readable for RMONCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmoncnt::W](W) writer structure"]
impl crate::Writable for RMONCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMONCNT to value 0"]
impl crate::Resettable for RMONCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
