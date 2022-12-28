#[doc = "Register `SDIO_MODE` reader"]
pub struct R(crate::R<SDIO_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_MODE` writer"]
pub struct W(crate::W<SDIO_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_MODE_SPEC>;
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
impl From<crate::W<SDIO_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN` reader - SDIO Mode"]
pub type INTEN_R = crate::BitReader<INTEN_A>;
#[doc = "SDIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTEN_A {
    #[doc = "1: Enables the SD host interface to receive SDIO interrupt from the SDIO card"]
    _1 = 1,
    #[doc = "0: Disables the SD host interface to receive SDIO interrupt from the SDIO card"]
    _0 = 0,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl INTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            true => INTEN_A::_1,
            false => INTEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTEN_A::_0
    }
}
#[doc = "Field `INTEN` writer - SDIO Mode"]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_MODE_SPEC, INTEN_A, O>;
impl<'a, const O: u8> INTEN_W<'a, O> {
    #[doc = "Enables the SD host interface to receive SDIO interrupt from the SDIO card"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN_A::_1)
    }
    #[doc = "Disables the SD host interface to receive SDIO interrupt from the SDIO card"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN_A::_0)
    }
}
#[doc = "Field `RWREQ` reader - Read Wait Request"]
pub type RWREQ_R = crate::BitReader<RWREQ_A>;
#[doc = "Read Wait Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWREQ_A {
    #[doc = "0: Allow SD/MMC to exit read wait state"]
    _0 = 0,
    #[doc = "1: Request for SD/MMC to enter read wait state."]
    _1 = 1,
}
impl From<RWREQ_A> for bool {
    #[inline(always)]
    fn from(variant: RWREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl RWREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWREQ_A {
        match self.bits {
            false => RWREQ_A::_0,
            true => RWREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWREQ_A::_1
    }
}
#[doc = "Field `RWREQ` writer - Read Wait Request"]
pub type RWREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_MODE_SPEC, RWREQ_A, O>;
impl<'a, const O: u8> RWREQ_W<'a, O> {
    #[doc = "Allow SD/MMC to exit read wait state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWREQ_A::_0)
    }
    #[doc = "Request for SD/MMC to enter read wait state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWREQ_A::_1)
    }
}
#[doc = "Field `IOABT` reader - SDIO Abort NOTE: See manual"]
pub type IOABT_R = crate::BitReader<bool>;
#[doc = "Field `IOABT` writer - SDIO Abort NOTE: See manual"]
pub type IOABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_MODE_SPEC, bool, O>;
#[doc = "Field `C52PUB` reader - SDIO None Abort NOTE: See manual"]
pub type C52PUB_R = crate::BitReader<bool>;
#[doc = "Field `C52PUB` writer - SDIO None Abort NOTE: See manual"]
pub type C52PUB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SDIO Mode"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Read Wait Request"]
    #[inline(always)]
    pub fn rwreq(&self) -> RWREQ_R {
        RWREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO Abort NOTE: See manual"]
    #[inline(always)]
    pub fn ioabt(&self) -> IOABT_R {
        IOABT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDIO None Abort NOTE: See manual"]
    #[inline(always)]
    pub fn c52pub(&self) -> C52PUB_R {
        C52PUB_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDIO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<0> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 2 - Read Wait Request"]
    #[inline(always)]
    #[must_use]
    pub fn rwreq(&mut self) -> RWREQ_W<2> {
        RWREQ_W::new(self)
    }
    #[doc = "Bit 8 - SDIO Abort NOTE: See manual"]
    #[inline(always)]
    #[must_use]
    pub fn ioabt(&mut self) -> IOABT_W<8> {
        IOABT_W::new(self)
    }
    #[doc = "Bit 9 - SDIO None Abort NOTE: See manual"]
    #[inline(always)]
    #[must_use]
    pub fn c52pub(&mut self) -> C52PUB_W<9> {
        C52PUB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_mode](index.html) module"]
pub struct SDIO_MODE_SPEC;
impl crate::RegisterSpec for SDIO_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_mode::R](R) reader structure"]
impl crate::Readable for SDIO_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_mode::W](W) writer structure"]
impl crate::Writable for SDIO_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_MODE to value 0"]
impl crate::Resettable for SDIO_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
