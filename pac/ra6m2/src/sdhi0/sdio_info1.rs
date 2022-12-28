#[doc = "Register `SDIO_INFO1` reader"]
pub struct R(crate::R<SDIO_INFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_INFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_INFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_INFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_INFO1` writer"]
pub struct W(crate::W<SDIO_INFO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_INFO1_SPEC>;
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
impl From<crate::W<SDIO_INFO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_INFO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOIRQ` reader - SDIO Interrupt Status\n\nThe field is **modified** in some way after a read operation."]
pub type IOIRQ_R = crate::BitReader<IOIRQ_A>;
#[doc = "SDIO Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOIRQ_A {
    #[doc = "0: SDIO interrupt not accepted"]
    _0 = 0,
    #[doc = "1: SDIO interrupt accepted"]
    _1 = 1,
}
impl From<IOIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: IOIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IOIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOIRQ_A {
        match self.bits {
            false => IOIRQ_A::_0,
            true => IOIRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOIRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOIRQ_A::_1
    }
}
#[doc = "Field `IOIRQ` writer - SDIO Interrupt Status"]
pub type IOIRQ_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SDIO_INFO1_SPEC, IOIRQ_A, O>;
impl<'a, const O: u8> IOIRQ_W<'a, O> {
    #[doc = "SDIO interrupt not accepted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IOIRQ_A::_0)
    }
    #[doc = "SDIO interrupt accepted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IOIRQ_A::_1)
    }
}
#[doc = "Field `EXPUB52` reader - EXPUB52 Status FlagNOTE: See manual\n\nThe field is **modified** in some way after a read operation."]
pub type EXPUB52_R = crate::BitReader<bool>;
#[doc = "Field `EXPUB52` writer - EXPUB52 Status FlagNOTE: See manual"]
pub type EXPUB52_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SDIO_INFO1_SPEC, bool, O>;
#[doc = "Field `EXWT` reader - EXWT Status FlagNOTE: See manual\n\nThe field is **modified** in some way after a read operation."]
pub type EXWT_R = crate::BitReader<bool>;
#[doc = "Field `EXWT` writer - EXWT Status FlagNOTE: See manual"]
pub type EXWT_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SDIO_INFO1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SDIO Interrupt Status"]
    #[inline(always)]
    pub fn ioirq(&self) -> IOIRQ_R {
        IOIRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 14 - EXPUB52 Status FlagNOTE: See manual"]
    #[inline(always)]
    pub fn expub52(&self) -> EXPUB52_R {
        EXPUB52_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXWT Status FlagNOTE: See manual"]
    #[inline(always)]
    pub fn exwt(&self) -> EXWT_R {
        EXWT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDIO Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn ioirq(&mut self) -> IOIRQ_W<0> {
        IOIRQ_W::new(self)
    }
    #[doc = "Bit 14 - EXPUB52 Status FlagNOTE: See manual"]
    #[inline(always)]
    #[must_use]
    pub fn expub52(&mut self) -> EXPUB52_W<14> {
        EXPUB52_W::new(self)
    }
    #[doc = "Bit 15 - EXWT Status FlagNOTE: See manual"]
    #[inline(always)]
    #[must_use]
    pub fn exwt(&mut self) -> EXWT_W<15> {
        EXWT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO Interrupt Flag Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_info1](index.html) module"]
pub struct SDIO_INFO1_SPEC;
impl crate::RegisterSpec for SDIO_INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_info1::R](R) reader structure"]
impl crate::Readable for SDIO_INFO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_info1::W](W) writer structure"]
impl crate::Writable for SDIO_INFO1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xc001;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_INFO1 to value 0"]
impl crate::Resettable for SDIO_INFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
