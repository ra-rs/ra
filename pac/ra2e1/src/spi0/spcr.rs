#[doc = "Register `SPCR` reader"]
pub struct R(crate::R<SPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCR` writer"]
pub struct W(crate::W<SPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCR_SPEC>;
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
impl From<crate::W<SPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPMS` reader - SPI Mode Select"]
pub type SPMS_R = crate::BitReader<SPMS_A>;
#[doc = "SPI Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPMS_A {
    #[doc = "0: Select SPI operation (4-wire method)"]
    _0 = 0,
    #[doc = "1: Select clock synchronous operation (3-wire method)"]
    _1 = 1,
}
impl From<SPMS_A> for bool {
    #[inline(always)]
    fn from(variant: SPMS_A) -> Self {
        variant as u8 != 0
    }
}
impl SPMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPMS_A {
        match self.bits {
            false => SPMS_A::_0,
            true => SPMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPMS_A::_1
    }
}
#[doc = "Field `SPMS` writer - SPI Mode Select"]
pub type SPMS_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, SPMS_A, O>;
impl<'a, const O: u8> SPMS_W<'a, O> {
    #[doc = "Select SPI operation (4-wire method)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPMS_A::_0)
    }
    #[doc = "Select clock synchronous operation (3-wire method)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPMS_A::_1)
    }
}
#[doc = "Field `TXMD` reader - Communications Operating Mode Select"]
pub type TXMD_R = crate::BitReader<TXMD_A>;
#[doc = "Communications Operating Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMD_A {
    #[doc = "0: Select full-duplex synchronous serial communications"]
    _0 = 0,
    #[doc = "1: Select serial communications with transmit-only"]
    _1 = 1,
}
impl From<TXMD_A> for bool {
    #[inline(always)]
    fn from(variant: TXMD_A) -> Self {
        variant as u8 != 0
    }
}
impl TXMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXMD_A {
        match self.bits {
            false => TXMD_A::_0,
            true => TXMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXMD_A::_1
    }
}
#[doc = "Field `TXMD` writer - Communications Operating Mode Select"]
pub type TXMD_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, TXMD_A, O>;
impl<'a, const O: u8> TXMD_W<'a, O> {
    #[doc = "Select full-duplex synchronous serial communications"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXMD_A::_0)
    }
    #[doc = "Select serial communications with transmit-only"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXMD_A::_1)
    }
}
#[doc = "Field `MODFEN` reader - Mode Fault Error Detection Enable"]
pub type MODFEN_R = crate::BitReader<MODFEN_A>;
#[doc = "Mode Fault Error Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODFEN_A {
    #[doc = "0: Disable detection of mode fault errors"]
    _0 = 0,
    #[doc = "1: Enable detection of mode fault errors"]
    _1 = 1,
}
impl From<MODFEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MODFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODFEN_A {
        match self.bits {
            false => MODFEN_A::_0,
            true => MODFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODFEN_A::_1
    }
}
#[doc = "Field `MODFEN` writer - Mode Fault Error Detection Enable"]
pub type MODFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, MODFEN_A, O>;
impl<'a, const O: u8> MODFEN_W<'a, O> {
    #[doc = "Disable detection of mode fault errors"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODFEN_A::_0)
    }
    #[doc = "Enable detection of mode fault errors"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODFEN_A::_1)
    }
}
#[doc = "Field `MSTR` reader - SPI Master/Slave Mode Select"]
pub type MSTR_R = crate::BitReader<MSTR_A>;
#[doc = "SPI Master/Slave Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR_A {
    #[doc = "0: Select slave mode"]
    _0 = 0,
    #[doc = "1: Select master mode"]
    _1 = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::_0,
            true => MSTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTR_A::_1
    }
}
#[doc = "Field `MSTR` writer - SPI Master/Slave Mode Select"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, MSTR_A, O>;
impl<'a, const O: u8> MSTR_W<'a, O> {
    #[doc = "Select slave mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTR_A::_0)
    }
    #[doc = "Select master mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTR_A::_1)
    }
}
#[doc = "Field `SPEIE` reader - SPI Error Interrupt Enable"]
pub type SPEIE_R = crate::BitReader<SPEIE_A>;
#[doc = "SPI Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEIE_A {
    #[doc = "0: Disable SPI error interrupt requests"]
    _0 = 0,
    #[doc = "1: Enable SPI error interrupt requests"]
    _1 = 1,
}
impl From<SPEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEIE_A {
        match self.bits {
            false => SPEIE_A::_0,
            true => SPEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPEIE_A::_1
    }
}
#[doc = "Field `SPEIE` writer - SPI Error Interrupt Enable"]
pub type SPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, SPEIE_A, O>;
impl<'a, const O: u8> SPEIE_W<'a, O> {
    #[doc = "Disable SPI error interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPEIE_A::_0)
    }
    #[doc = "Enable SPI error interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPEIE_A::_1)
    }
}
#[doc = "Field `SPTIE` reader - Transmit Buffer Empty Interrupt Enable"]
pub type SPTIE_R = crate::BitReader<SPTIE_A>;
#[doc = "Transmit Buffer Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPTIE_A {
    #[doc = "0: Disable transmit buffer empty interrupt requests"]
    _0 = 0,
    #[doc = "1: Enable transmit buffer empty interrupt requests"]
    _1 = 1,
}
impl From<SPTIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTIE_A {
        match self.bits {
            false => SPTIE_A::_0,
            true => SPTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPTIE_A::_1
    }
}
#[doc = "Field `SPTIE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type SPTIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, SPTIE_A, O>;
impl<'a, const O: u8> SPTIE_W<'a, O> {
    #[doc = "Disable transmit buffer empty interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPTIE_A::_0)
    }
    #[doc = "Enable transmit buffer empty interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPTIE_A::_1)
    }
}
#[doc = "Field `SPE` reader - SPI Function Enable"]
pub type SPE_R = crate::BitReader<SPE_A>;
#[doc = "SPI Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPE_A {
    #[doc = "0: Disable SPI function"]
    _0 = 0,
    #[doc = "1: Enable SPI function"]
    _1 = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::_0,
            true => SPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPE_A::_1
    }
}
#[doc = "Field `SPE` writer - SPI Function Enable"]
pub type SPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, SPE_A, O>;
impl<'a, const O: u8> SPE_W<'a, O> {
    #[doc = "Disable SPI function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPE_A::_0)
    }
    #[doc = "Enable SPI function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPE_A::_1)
    }
}
#[doc = "Field `SPRIE` reader - SPI Receive Buffer Full Interrupt Enable"]
pub type SPRIE_R = crate::BitReader<SPRIE_A>;
#[doc = "SPI Receive Buffer Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPRIE_A {
    #[doc = "0: Disable SPI receive buffer full interrupt requests"]
    _0 = 0,
    #[doc = "1: Enable SPI receive buffer full interrupt requests"]
    _1 = 1,
}
impl From<SPRIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRIE_A {
        match self.bits {
            false => SPRIE_A::_0,
            true => SPRIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRIE_A::_1
    }
}
#[doc = "Field `SPRIE` writer - SPI Receive Buffer Full Interrupt Enable"]
pub type SPRIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPCR_SPEC, SPRIE_A, O>;
impl<'a, const O: u8> SPRIE_W<'a, O> {
    #[doc = "Disable SPI receive buffer full interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPRIE_A::_0)
    }
    #[doc = "Enable SPI receive buffer full interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPRIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SPI Mode Select"]
    #[inline(always)]
    pub fn spms(&self) -> SPMS_R {
        SPMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Communications Operating Mode Select"]
    #[inline(always)]
    pub fn txmd(&self) -> TXMD_R {
        TXMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error Detection Enable"]
    #[inline(always)]
    pub fn modfen(&self) -> MODFEN_R {
        MODFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI Error Interrupt Enable"]
    #[inline(always)]
    pub fn speie(&self) -> SPEIE_R {
        SPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn sptie(&self) -> SPTIE_R {
        SPTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Function Enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn sprie(&self) -> SPRIE_R {
        SPRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Mode Select"]
    #[inline(always)]
    pub fn spms(&mut self) -> SPMS_W<0> {
        SPMS_W::new(self)
    }
    #[doc = "Bit 1 - Communications Operating Mode Select"]
    #[inline(always)]
    pub fn txmd(&mut self) -> TXMD_W<1> {
        TXMD_W::new(self)
    }
    #[doc = "Bit 2 - Mode Fault Error Detection Enable"]
    #[inline(always)]
    pub fn modfen(&mut self) -> MODFEN_W<2> {
        MODFEN_W::new(self)
    }
    #[doc = "Bit 3 - SPI Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<3> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 4 - SPI Error Interrupt Enable"]
    #[inline(always)]
    pub fn speie(&mut self) -> SPEIE_W<4> {
        SPEIE_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn sptie(&mut self) -> SPTIE_W<5> {
        SPTIE_W::new(self)
    }
    #[doc = "Bit 6 - SPI Function Enable"]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<6> {
        SPE_W::new(self)
    }
    #[doc = "Bit 7 - SPI Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn sprie(&mut self) -> SPRIE_W<7> {
        SPRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcr](index.html) module"]
pub struct SPCR_SPEC;
impl crate::RegisterSpec for SPCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spcr::R](R) reader structure"]
impl crate::Readable for SPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcr::W](W) writer structure"]
impl crate::Writable for SPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPCR to value 0"]
impl crate::Resettable for SPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
