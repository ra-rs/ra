#[doc = "Register `BCNT2AER` reader"]
pub struct R(crate::R<BCNT2AER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCNT2AER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCNT2AER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCNT2AER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCNT2AER` writer"]
pub struct W(crate::W<BCNT2AER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCNT2AER_SPEC>;
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
impl From<crate::W<BCNT2AER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCNT2AER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENB` reader - Setting the alarm enable associated with the 32-bit binary counter"]
pub type ENB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENB` writer - Setting the alarm enable associated with the 32-bit binary counter"]
pub type ENB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, BCNT2AER_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Setting the alarm enable associated with the 32-bit binary counter"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Setting the alarm enable associated with the 32-bit binary counter"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W<0> {
        ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Binary Counter 2 Alarm Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcnt2aer](index.html) module"]
pub struct BCNT2AER_SPEC;
impl crate::RegisterSpec for BCNT2AER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [bcnt2aer::R](R) reader structure"]
impl crate::Readable for BCNT2AER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcnt2aer::W](W) writer structure"]
impl crate::Writable for BCNT2AER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCNT2AER to value 0"]
impl crate::Resettable for BCNT2AER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
