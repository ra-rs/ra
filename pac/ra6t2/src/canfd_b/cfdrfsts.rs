#[doc = "Register `CFDRFSTS%s` reader"]
pub struct R(crate::R<CFDRFSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDRFSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDRFSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDRFSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDRFSTS%s` writer"]
pub struct W(crate::W<CFDRFSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDRFSTS_SPEC>;
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
impl From<crate::W<CFDRFSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDRFSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFEMP` reader - RX FIFO Empty"]
pub type RFEMP_R = crate::BitReader<RFEMP_A>;
#[doc = "RX FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEMP_A {
    #[doc = "0: FIFO not empty"]
    _0 = 0,
    #[doc = "1: FIFO empty"]
    _1 = 1,
}
impl From<RFEMP_A> for bool {
    #[inline(always)]
    fn from(variant: RFEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl RFEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEMP_A {
        match self.bits {
            false => RFEMP_A::_0,
            true => RFEMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFEMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFEMP_A::_1
    }
}
#[doc = "Field `RFFLL` reader - RX FIFO Full"]
pub type RFFLL_R = crate::BitReader<RFFLL_A>;
#[doc = "RX FIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFFLL_A {
    #[doc = "0: FIFO not full"]
    _0 = 0,
    #[doc = "1: FIFO full"]
    _1 = 1,
}
impl From<RFFLL_A> for bool {
    #[inline(always)]
    fn from(variant: RFFLL_A) -> Self {
        variant as u8 != 0
    }
}
impl RFFLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFFLL_A {
        match self.bits {
            false => RFFLL_A::_0,
            true => RFFLL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFFLL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFFLL_A::_1
    }
}
#[doc = "Field `RFMLT` reader - RX FIFO Message Lost"]
pub type RFMLT_R = crate::BitReader<RFMLT_A>;
#[doc = "RX FIFO Message Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFMLT_A {
    #[doc = "0: No message lost in FIFO"]
    _0 = 0,
    #[doc = "1: FIFO message lost"]
    _1 = 1,
}
impl From<RFMLT_A> for bool {
    #[inline(always)]
    fn from(variant: RFMLT_A) -> Self {
        variant as u8 != 0
    }
}
impl RFMLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFMLT_A {
        match self.bits {
            false => RFMLT_A::_0,
            true => RFMLT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFMLT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFMLT_A::_1
    }
}
#[doc = "Field `RFMLT` writer - RX FIFO Message Lost"]
pub type RFMLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDRFSTS_SPEC, RFMLT_A, O>;
impl<'a, const O: u8> RFMLT_W<'a, O> {
    #[doc = "No message lost in FIFO"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFMLT_A::_0)
    }
    #[doc = "FIFO message lost"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFMLT_A::_1)
    }
}
#[doc = "Field `RFIF` reader - RX FIFO Interrupt Flag"]
pub type RFIF_R = crate::BitReader<RFIF_A>;
#[doc = "RX FIFO Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFIF_A {
    #[doc = "0: FIFO interrupt condition not satisfied"]
    _0 = 0,
    #[doc = "1: FIFO interrupt condition satisfied"]
    _1 = 1,
}
impl From<RFIF_A> for bool {
    #[inline(always)]
    fn from(variant: RFIF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFIF_A {
        match self.bits {
            false => RFIF_A::_0,
            true => RFIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFIF_A::_1
    }
}
#[doc = "Field `RFIF` writer - RX FIFO Interrupt Flag"]
pub type RFIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDRFSTS_SPEC, RFIF_A, O>;
impl<'a, const O: u8> RFIF_W<'a, O> {
    #[doc = "FIFO interrupt condition not satisfied"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFIF_A::_0)
    }
    #[doc = "FIFO interrupt condition satisfied"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFIF_A::_1)
    }
}
#[doc = "Field `RFMC` reader - RX FIFO Message Count"]
pub type RFMC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - RX FIFO Empty"]
    #[inline(always)]
    pub fn rfemp(&self) -> RFEMP_R {
        RFEMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Full"]
    #[inline(always)]
    pub fn rffll(&self) -> RFFLL_R {
        RFFLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Message Lost"]
    #[inline(always)]
    pub fn rfmlt(&self) -> RFMLT_R {
        RFMLT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Interrupt Flag"]
    #[inline(always)]
    pub fn rfif(&self) -> RFIF_R {
        RFIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:13 - RX FIFO Message Count"]
    #[inline(always)]
    pub fn rfmc(&self) -> RFMC_R {
        RFMC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - RX FIFO Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rfmlt(&mut self) -> RFMLT_W<2> {
        RFMLT_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfif(&mut self) -> RFIF_W<3> {
        RFIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX FIFO Status Registers %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdrfsts](index.html) module"]
pub struct CFDRFSTS_SPEC;
impl crate::RegisterSpec for CFDRFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdrfsts::R](R) reader structure"]
impl crate::Readable for CFDRFSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdrfsts::W](W) writer structure"]
impl crate::Writable for CFDRFSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDRFSTS%s to value 0x01"]
impl crate::Resettable for CFDRFSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
