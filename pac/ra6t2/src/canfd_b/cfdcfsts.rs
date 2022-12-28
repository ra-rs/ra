#[doc = "Register `CFDCFSTS` reader"]
pub struct R(crate::R<CFDCFSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCFSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDCFSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDCFSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDCFSTS` writer"]
pub struct W(crate::W<CFDCFSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCFSTS_SPEC>;
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
impl From<crate::W<CFDCFSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDCFSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFEMP` reader - Common FIFO Empty"]
pub type CFEMP_R = crate::BitReader<CFEMP_A>;
#[doc = "Common FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFEMP_A {
    #[doc = "0: FIFO not empty"]
    _0 = 0,
    #[doc = "1: FIFO empty"]
    _1 = 1,
}
impl From<CFEMP_A> for bool {
    #[inline(always)]
    fn from(variant: CFEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl CFEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFEMP_A {
        match self.bits {
            false => CFEMP_A::_0,
            true => CFEMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFEMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFEMP_A::_1
    }
}
#[doc = "Field `CFFLL` reader - Common FIFO Full"]
pub type CFFLL_R = crate::BitReader<CFFLL_A>;
#[doc = "Common FIFO Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFFLL_A {
    #[doc = "0: FIFO not full"]
    _0 = 0,
    #[doc = "1: FIFO full"]
    _1 = 1,
}
impl From<CFFLL_A> for bool {
    #[inline(always)]
    fn from(variant: CFFLL_A) -> Self {
        variant as u8 != 0
    }
}
impl CFFLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFFLL_A {
        match self.bits {
            false => CFFLL_A::_0,
            true => CFFLL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFFLL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFFLL_A::_1
    }
}
#[doc = "Field `CFMLT` reader - Common FIFO Message Lost"]
pub type CFMLT_R = crate::BitReader<CFMLT_A>;
#[doc = "Common FIFO Message Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFMLT_A {
    #[doc = "0: Number of message lost in FIFO"]
    _0 = 0,
    #[doc = "1: FIFO message lost"]
    _1 = 1,
}
impl From<CFMLT_A> for bool {
    #[inline(always)]
    fn from(variant: CFMLT_A) -> Self {
        variant as u8 != 0
    }
}
impl CFMLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFMLT_A {
        match self.bits {
            false => CFMLT_A::_0,
            true => CFMLT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFMLT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFMLT_A::_1
    }
}
#[doc = "Field `CFMLT` writer - Common FIFO Message Lost"]
pub type CFMLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFSTS_SPEC, CFMLT_A, O>;
impl<'a, const O: u8> CFMLT_W<'a, O> {
    #[doc = "Number of message lost in FIFO"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFMLT_A::_0)
    }
    #[doc = "FIFO message lost"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFMLT_A::_1)
    }
}
#[doc = "Field `CFRXIF` reader - Common RX FIFO Interrupt Flag"]
pub type CFRXIF_R = crate::BitReader<CFRXIF_A>;
#[doc = "Common RX FIFO Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFRXIF_A {
    #[doc = "0: FIFO interrupt condition not satisfied after frame reception"]
    _0 = 0,
    #[doc = "1: FIFO interrupt condition satisfied after frame reception"]
    _1 = 1,
}
impl From<CFRXIF_A> for bool {
    #[inline(always)]
    fn from(variant: CFRXIF_A) -> Self {
        variant as u8 != 0
    }
}
impl CFRXIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFRXIF_A {
        match self.bits {
            false => CFRXIF_A::_0,
            true => CFRXIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFRXIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFRXIF_A::_1
    }
}
#[doc = "Field `CFRXIF` writer - Common RX FIFO Interrupt Flag"]
pub type CFRXIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFSTS_SPEC, CFRXIF_A, O>;
impl<'a, const O: u8> CFRXIF_W<'a, O> {
    #[doc = "FIFO interrupt condition not satisfied after frame reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFRXIF_A::_0)
    }
    #[doc = "FIFO interrupt condition satisfied after frame reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFRXIF_A::_1)
    }
}
#[doc = "Field `CFTXIF` reader - Common TX FIFO Interrupt Flag"]
pub type CFTXIF_R = crate::BitReader<CFTXIF_A>;
#[doc = "Common TX FIFO Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFTXIF_A {
    #[doc = "0: FIFO interrupt condition not satisfied after frame transmission"]
    _0 = 0,
    #[doc = "1: FIFO Interrupt condition satisfied after frame transmission"]
    _1 = 1,
}
impl From<CFTXIF_A> for bool {
    #[inline(always)]
    fn from(variant: CFTXIF_A) -> Self {
        variant as u8 != 0
    }
}
impl CFTXIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFTXIF_A {
        match self.bits {
            false => CFTXIF_A::_0,
            true => CFTXIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFTXIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFTXIF_A::_1
    }
}
#[doc = "Field `CFTXIF` writer - Common TX FIFO Interrupt Flag"]
pub type CFTXIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCFSTS_SPEC, CFTXIF_A, O>;
impl<'a, const O: u8> CFTXIF_W<'a, O> {
    #[doc = "FIFO interrupt condition not satisfied after frame transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFTXIF_A::_0)
    }
    #[doc = "FIFO Interrupt condition satisfied after frame transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFTXIF_A::_1)
    }
}
#[doc = "Field `CFMC` reader - Common FIFO Message Count"]
pub type CFMC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Common FIFO Empty"]
    #[inline(always)]
    pub fn cfemp(&self) -> CFEMP_R {
        CFEMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Common FIFO Full"]
    #[inline(always)]
    pub fn cffll(&self) -> CFFLL_R {
        CFFLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Common FIFO Message Lost"]
    #[inline(always)]
    pub fn cfmlt(&self) -> CFMLT_R {
        CFMLT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Common RX FIFO Interrupt Flag"]
    #[inline(always)]
    pub fn cfrxif(&self) -> CFRXIF_R {
        CFRXIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Common TX FIFO Interrupt Flag"]
    #[inline(always)]
    pub fn cftxif(&self) -> CFTXIF_R {
        CFTXIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Common FIFO Message Count"]
    #[inline(always)]
    pub fn cfmc(&self) -> CFMC_R {
        CFMC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Common FIFO Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn cfmlt(&mut self) -> CFMLT_W<2> {
        CFMLT_W::new(self)
    }
    #[doc = "Bit 3 - Common RX FIFO Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfrxif(&mut self) -> CFRXIF_W<3> {
        CFRXIF_W::new(self)
    }
    #[doc = "Bit 4 - Common TX FIFO Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cftxif(&mut self) -> CFTXIF_W<4> {
        CFTXIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdcfsts](index.html) module"]
pub struct CFDCFSTS_SPEC;
impl crate::RegisterSpec for CFDCFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdcfsts::R](R) reader structure"]
impl crate::Readable for CFDCFSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdcfsts::W](W) writer structure"]
impl crate::Writable for CFDCFSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDCFSTS to value 0x01"]
impl crate::Resettable for CFDCFSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
