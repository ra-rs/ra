#[doc = "Register `PIPE%sTRN` reader"]
pub struct R(crate::R<PIPETRN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIPETRN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIPETRN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIPETRN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIPE%sTRN` writer"]
pub struct W(crate::W<PIPETRN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIPETRN_SPEC>;
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
impl From<crate::W<PIPETRN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIPETRN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRNCNT` reader - Transaction CounterWhen writing to: Specify the number of total packets (number of transactions) to be received by the relevant PIPE.When read from: When TRENB = 0: Indicate the specified number of transactions.When TRENB = 1: Indicate the number of currently counted transactions."]
pub type TRNCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRNCNT` writer - Transaction CounterWhen writing to: Specify the number of total packets (number of transactions) to be received by the relevant PIPE.When read from: When TRENB = 0: Indicate the specified number of transactions.When TRENB = 1: Indicate the number of currently counted transactions."]
pub type TRNCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PIPETRN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Transaction CounterWhen writing to: Specify the number of total packets (number of transactions) to be received by the relevant PIPE.When read from: When TRENB = 0: Indicate the specified number of transactions.When TRENB = 1: Indicate the number of currently counted transactions."]
    #[inline(always)]
    pub fn trncnt(&self) -> TRNCNT_R {
        TRNCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transaction CounterWhen writing to: Specify the number of total packets (number of transactions) to be received by the relevant PIPE.When read from: When TRENB = 0: Indicate the specified number of transactions.When TRENB = 1: Indicate the number of currently counted transactions."]
    #[inline(always)]
    #[must_use]
    pub fn trncnt(&mut self) -> TRNCNT_W<0> {
        TRNCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIPE Transaction Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pipetrn](index.html) module"]
pub struct PIPETRN_SPEC;
impl crate::RegisterSpec for PIPETRN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pipetrn::R](R) reader structure"]
impl crate::Readable for PIPETRN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pipetrn::W](W) writer structure"]
impl crate::Writable for PIPETRN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPE%sTRN to value 0"]
impl crate::Resettable for PIPETRN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
