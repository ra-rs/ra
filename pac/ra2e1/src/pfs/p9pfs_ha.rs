#[doc = "Register `P9%sPFS_HA` reader"]
pub struct R(crate::R<P9PFS_HA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P9PFS_HA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P9PFS_HA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P9PFS_HA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P9%sPFS_HA` writer"]
pub struct W(crate::W<P9PFS_HA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P9PFS_HA_SPEC>;
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
impl From<crate::W<P9PFS_HA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P9PFS_HA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PODR` reader - Port Output Data"]
pub type PODR_R = crate::BitReader<PODR_A>;
#[doc = "Port Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PODR_A {
    #[doc = "0: Output low"]
    _0 = 0,
    #[doc = "1: Output high"]
    _1 = 1,
}
impl From<PODR_A> for bool {
    #[inline(always)]
    fn from(variant: PODR_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PODR_A {
        match self.bits {
            false => PODR_A::_0,
            true => PODR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR_A::_1
    }
}
#[doc = "Field `PODR` writer - Port Output Data"]
pub type PODR_W<'a, const O: u8> = crate::BitWriter<'a, u16, P9PFS_HA_SPEC, PODR_A, O>;
impl<'a, const O: u8> PODR_W<'a, O> {
    #[doc = "Output low"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PODR_A::_0)
    }
    #[doc = "Output high"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PODR_A::_1)
    }
}
#[doc = "Field `PIDR` reader - Port State"]
pub type PIDR_R = crate::BitReader<PIDR_A>;
#[doc = "Port State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR_A {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<PIDR_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDR_A {
        match self.bits {
            false => PIDR_A::_0,
            true => PIDR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR_A::_1
    }
}
#[doc = "Field `PDR` reader - Port Direction"]
pub type PDR_R = crate::BitReader<PDR_A>;
#[doc = "Port Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR_A {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<PDR_A> for bool {
    #[inline(always)]
    fn from(variant: PDR_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDR_A {
        match self.bits {
            false => PDR_A::_0,
            true => PDR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR_A::_1
    }
}
#[doc = "Field `PDR` writer - Port Direction"]
pub type PDR_W<'a, const O: u8> = crate::BitWriter<'a, u16, P9PFS_HA_SPEC, PDR_A, O>;
impl<'a, const O: u8> PDR_W<'a, O> {
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDR_A::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDR_A::_1)
    }
}
#[doc = "Field `PCR` reader - Pull-up Control"]
pub type PCR_R = crate::BitReader<PCR_A>;
#[doc = "Pull-up Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCR_A {
    #[doc = "0: Disable input pull-up"]
    _0 = 0,
    #[doc = "1: Enable input pull-up"]
    _1 = 1,
}
impl From<PCR_A> for bool {
    #[inline(always)]
    fn from(variant: PCR_A) -> Self {
        variant as u8 != 0
    }
}
impl PCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCR_A {
        match self.bits {
            false => PCR_A::_0,
            true => PCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCR_A::_1
    }
}
#[doc = "Field `PCR` writer - Pull-up Control"]
pub type PCR_W<'a, const O: u8> = crate::BitWriter<'a, u16, P9PFS_HA_SPEC, PCR_A, O>;
impl<'a, const O: u8> PCR_W<'a, O> {
    #[doc = "Disable input pull-up"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCR_A::_0)
    }
    #[doc = "Enable input pull-up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCR_A::_1)
    }
}
#[doc = "Field `NCODR` reader - N-Channel Open-Drain Control"]
pub type NCODR_R = crate::BitReader<NCODR_A>;
#[doc = "N-Channel Open-Drain Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCODR_A {
    #[doc = "0: Output CMOS"]
    _0 = 0,
    #[doc = "1: Output NMOS open-drain"]
    _1 = 1,
}
impl From<NCODR_A> for bool {
    #[inline(always)]
    fn from(variant: NCODR_A) -> Self {
        variant as u8 != 0
    }
}
impl NCODR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCODR_A {
        match self.bits {
            false => NCODR_A::_0,
            true => NCODR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NCODR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NCODR_A::_1
    }
}
#[doc = "Field `NCODR` writer - N-Channel Open-Drain Control"]
pub type NCODR_W<'a, const O: u8> = crate::BitWriter<'a, u16, P9PFS_HA_SPEC, NCODR_A, O>;
impl<'a, const O: u8> NCODR_W<'a, O> {
    #[doc = "Output CMOS"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NCODR_A::_0)
    }
    #[doc = "Output NMOS open-drain"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NCODR_A::_1)
    }
}
#[doc = "Field `ISEL` reader - IRQ Input Enable"]
pub type ISEL_R = crate::BitReader<ISEL_A>;
#[doc = "IRQ Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISEL_A {
    #[doc = "0: Do not use as IRQn input pin"]
    _0 = 0,
    #[doc = "1: Use as IRQn input pin"]
    _1 = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::_0,
            true => ISEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISEL_A::_1
    }
}
#[doc = "Field `ISEL` writer - IRQ Input Enable"]
pub type ISEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, P9PFS_HA_SPEC, ISEL_A, O>;
impl<'a, const O: u8> ISEL_W<'a, O> {
    #[doc = "Do not use as IRQn input pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISEL_A::_0)
    }
    #[doc = "Use as IRQn input pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISEL_A::_1)
    }
}
#[doc = "Field `ASEL` reader - Analog Input Enable"]
pub type ASEL_R = crate::BitReader<ASEL_A>;
#[doc = "Analog Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASEL_A {
    #[doc = "0: Do not use as analog pin"]
    _0 = 0,
    #[doc = "1: Use as analog pin"]
    _1 = 1,
}
impl From<ASEL_A> for bool {
    #[inline(always)]
    fn from(variant: ASEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASEL_A {
        match self.bits {
            false => ASEL_A::_0,
            true => ASEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASEL_A::_1
    }
}
#[doc = "Field `ASEL` writer - Analog Input Enable"]
pub type ASEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, P9PFS_HA_SPEC, ASEL_A, O>;
impl<'a, const O: u8> ASEL_W<'a, O> {
    #[doc = "Do not use as analog pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASEL_A::_0)
    }
    #[doc = "Use as analog pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    pub fn podr(&self) -> PODR_R {
        PODR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port State"]
    #[inline(always)]
    pub fn pidr(&self) -> PIDR_R {
        PIDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(&self) -> NCODR_R {
        NCODR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog Input Enable"]
    #[inline(always)]
    pub fn asel(&self) -> ASEL_R {
        ASEL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn podr(&mut self) -> PODR_W<0> {
        PODR_W::new(self)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr(&mut self) -> PDR_W<2> {
        PDR_W::new(self)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    #[must_use]
    pub fn pcr(&mut self) -> PCR_W<4> {
        PCR_W::new(self)
    }
    #[doc = "Bit 6 - N-Channel Open-Drain Control"]
    #[inline(always)]
    #[must_use]
    pub fn ncodr(&mut self) -> NCODR_W<6> {
        NCODR_W::new(self)
    }
    #[doc = "Bit 14 - IRQ Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn isel(&mut self) -> ISEL_W<14> {
        ISEL_W::new(self)
    }
    #[doc = "Bit 15 - Analog Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asel(&mut self) -> ASEL_W<15> {
        ASEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 9%s Pin Function Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9pfs_ha](index.html) module"]
pub struct P9PFS_HA_SPEC;
impl crate::RegisterSpec for P9PFS_HA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p9pfs_ha::R](R) reader structure"]
impl crate::Readable for P9PFS_HA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p9pfs_ha::W](W) writer structure"]
impl crate::Writable for P9PFS_HA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P9%sPFS_HA to value 0"]
impl crate::Resettable for P9PFS_HA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
