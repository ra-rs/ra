#[doc = "Register `FCFTR` reader"]
pub struct R(crate::R<FCFTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFTR` writer"]
pub struct W(crate::W<FCFTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFTR_SPEC>;
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
impl From<crate::W<FCFTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFDO` reader - Receive FIFO Data PAUSE Output Threshold"]
pub type RFDO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFDO` writer - Receive FIFO Data PAUSE Output Threshold"]
pub type RFDO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RFFO` reader - Receive FIFO Frame PAUSE Output Threshold"]
pub type RFFO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFFO` writer - Receive FIFO Frame PAUSE Output Threshold"]
pub type RFFO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCFTR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Receive FIFO Data PAUSE Output Threshold"]
    #[inline(always)]
    pub fn rfdo(&self) -> RFDO_R {
        RFDO_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - Receive FIFO Frame PAUSE Output Threshold"]
    #[inline(always)]
    pub fn rffo(&self) -> RFFO_R {
        RFFO_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive FIFO Data PAUSE Output Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rfdo(&mut self) -> RFDO_W<0> {
        RFDO_W::new(self)
    }
    #[doc = "Bits 16:18 - Receive FIFO Frame PAUSE Output Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rffo(&mut self) -> RFFO_W<16> {
        RFFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flow Control Start FIFO Threshold Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcftr](index.html) module"]
pub struct FCFTR_SPEC;
impl crate::RegisterSpec for FCFTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcftr::R](R) reader structure"]
impl crate::Readable for FCFTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcftr::W](W) writer structure"]
impl crate::Writable for FCFTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFTR to value 0x0007_0007"]
impl crate::Resettable for FCFTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0007;
}
