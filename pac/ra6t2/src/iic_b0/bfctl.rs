#[doc = "Register `BFCTL` reader"]
pub struct R(crate::R<BFCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFCTL` writer"]
pub struct W(crate::W<BFCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFCTL_SPEC>;
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
impl From<crate::W<BFCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MALE` reader - Master Arbitration-Lost Detection Enable"]
pub type MALE_R = crate::BitReader<MALE_A>;
#[doc = "Master Arbitration-Lost Detection Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MALE_A {
    #[doc = "0: Master arbitration-lost detection disables. Disables the arbitration-lost detection function and does not clear the CRMS and TRMD bits in PRSST automatically when arbitration is lost."]
    _0 = 0,
    #[doc = "1: Master arbitration-lost detection enables. Enables the arbitration-lost detection function and clears the CRMS and TRMD bits in PRSST automatically when arbitration is lost."]
    _1 = 1,
}
impl From<MALE_A> for bool {
    #[inline(always)]
    fn from(variant: MALE_A) -> Self {
        variant as u8 != 0
    }
}
impl MALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MALE_A {
        match self.bits {
            false => MALE_A::_0,
            true => MALE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MALE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MALE_A::_1
    }
}
#[doc = "Field `MALE` writer - Master Arbitration-Lost Detection Enable"]
pub type MALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFCTL_SPEC, MALE_A, O>;
impl<'a, const O: u8> MALE_W<'a, O> {
    #[doc = "Master arbitration-lost detection disables. Disables the arbitration-lost detection function and does not clear the CRMS and TRMD bits in PRSST automatically when arbitration is lost."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MALE_A::_0)
    }
    #[doc = "Master arbitration-lost detection enables. Enables the arbitration-lost detection function and clears the CRMS and TRMD bits in PRSST automatically when arbitration is lost."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MALE_A::_1)
    }
}
#[doc = "Field `NALE` reader - NACK Transmission Arbitration-Lost Detection Enable"]
pub type NALE_R = crate::BitReader<NALE_A>;
#[doc = "NACK Transmission Arbitration-Lost Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NALE_A {
    #[doc = "0: NACK transmission arbitration-lost detection disables."]
    _0 = 0,
    #[doc = "1: NACK transmission arbitration-lost detection enables."]
    _1 = 1,
}
impl From<NALE_A> for bool {
    #[inline(always)]
    fn from(variant: NALE_A) -> Self {
        variant as u8 != 0
    }
}
impl NALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NALE_A {
        match self.bits {
            false => NALE_A::_0,
            true => NALE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NALE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NALE_A::_1
    }
}
#[doc = "Field `NALE` writer - NACK Transmission Arbitration-Lost Detection Enable"]
pub type NALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFCTL_SPEC, NALE_A, O>;
impl<'a, const O: u8> NALE_W<'a, O> {
    #[doc = "NACK transmission arbitration-lost detection disables."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NALE_A::_0)
    }
    #[doc = "NACK transmission arbitration-lost detection enables."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NALE_A::_1)
    }
}
#[doc = "Field `SALE` reader - Slave Arbitration-Lost Detection Enable"]
pub type SALE_R = crate::BitReader<SALE_A>;
#[doc = "Slave Arbitration-Lost Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SALE_A {
    #[doc = "0: Slave arbitration-lost detection disables."]
    _0 = 0,
    #[doc = "1: Slave arbitration-lost detection enables."]
    _1 = 1,
}
impl From<SALE_A> for bool {
    #[inline(always)]
    fn from(variant: SALE_A) -> Self {
        variant as u8 != 0
    }
}
impl SALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SALE_A {
        match self.bits {
            false => SALE_A::_0,
            true => SALE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SALE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SALE_A::_1
    }
}
#[doc = "Field `SALE` writer - Slave Arbitration-Lost Detection Enable"]
pub type SALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFCTL_SPEC, SALE_A, O>;
impl<'a, const O: u8> SALE_W<'a, O> {
    #[doc = "Slave arbitration-lost detection disables."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SALE_A::_0)
    }
    #[doc = "Slave arbitration-lost detection enables."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SALE_A::_1)
    }
}
#[doc = "Field `SCSYNE` reader - SCL Synchronous Circuit Enable"]
pub type SCSYNE_R = crate::BitReader<SCSYNE_A>;
#[doc = "SCL Synchronous Circuit Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCSYNE_A {
    #[doc = "0: No SCL synchronous circuit uses."]
    _0 = 0,
    #[doc = "1: An SCL synchronous circuit uses."]
    _1 = 1,
}
impl From<SCSYNE_A> for bool {
    #[inline(always)]
    fn from(variant: SCSYNE_A) -> Self {
        variant as u8 != 0
    }
}
impl SCSYNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCSYNE_A {
        match self.bits {
            false => SCSYNE_A::_0,
            true => SCSYNE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCSYNE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCSYNE_A::_1
    }
}
#[doc = "Field `SCSYNE` writer - SCL Synchronous Circuit Enable"]
pub type SCSYNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFCTL_SPEC, SCSYNE_A, O>;
impl<'a, const O: u8> SCSYNE_W<'a, O> {
    #[doc = "No SCL synchronous circuit uses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCSYNE_A::_0)
    }
    #[doc = "An SCL synchronous circuit uses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCSYNE_A::_1)
    }
}
#[doc = "Field `SMBS` reader - SMBus/I2C Bus Selection"]
pub type SMBS_R = crate::BitReader<SMBS_A>;
#[doc = "SMBus/I2C Bus Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBS_A {
    #[doc = "0: The I2C bus select."]
    _0 = 0,
    #[doc = "1: The SMBus select."]
    _1 = 1,
}
impl From<SMBS_A> for bool {
    #[inline(always)]
    fn from(variant: SMBS_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBS_A {
        match self.bits {
            false => SMBS_A::_0,
            true => SMBS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMBS_A::_1
    }
}
#[doc = "Field `SMBS` writer - SMBus/I2C Bus Selection"]
pub type SMBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFCTL_SPEC, SMBS_A, O>;
impl<'a, const O: u8> SMBS_W<'a, O> {
    #[doc = "The I2C bus select."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMBS_A::_0)
    }
    #[doc = "The SMBus select."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMBS_A::_1)
    }
}
#[doc = "Field `FMPE` reader - Fast-mode Plus Enable"]
pub type FMPE_R = crate::BitReader<FMPE_A>;
#[doc = "Fast-mode Plus Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPE_A {
    #[doc = "0: No Fm+ slope control circuit uses for the SCLn pin and SDAn pin. (n = 0, 1)"]
    _0 = 0,
    #[doc = "1: An Fm+ slope control circuit uses for the SCLn pin and SDAn pin. (n = 0, 1)"]
    _1 = 1,
}
impl From<FMPE_A> for bool {
    #[inline(always)]
    fn from(variant: FMPE_A) -> Self {
        variant as u8 != 0
    }
}
impl FMPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMPE_A {
        match self.bits {
            false => FMPE_A::_0,
            true => FMPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FMPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FMPE_A::_1
    }
}
#[doc = "Field `FMPE` writer - Fast-mode Plus Enable"]
pub type FMPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFCTL_SPEC, FMPE_A, O>;
impl<'a, const O: u8> FMPE_W<'a, O> {
    #[doc = "No Fm+ slope control circuit uses for the SCLn pin and SDAn pin. (n = 0, 1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FMPE_A::_0)
    }
    #[doc = "An Fm+ slope control circuit uses for the SCLn pin and SDAn pin. (n = 0, 1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FMPE_A::_1)
    }
}
#[doc = "Field `HSME` reader - High Speed Mode Enable"]
pub type HSME_R = crate::BitReader<HSME_A>;
#[doc = "High Speed Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSME_A {
    #[doc = "0: Disable High Speed Mode."]
    _0 = 0,
    #[doc = "1: Enable High Speed Mode."]
    _1 = 1,
}
impl From<HSME_A> for bool {
    #[inline(always)]
    fn from(variant: HSME_A) -> Self {
        variant as u8 != 0
    }
}
impl HSME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSME_A {
        match self.bits {
            false => HSME_A::_0,
            true => HSME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSME_A::_1
    }
}
#[doc = "Field `HSME` writer - High Speed Mode Enable"]
pub type HSME_W<'a, const O: u8> = crate::BitWriter<'a, u32, BFCTL_SPEC, HSME_A, O>;
impl<'a, const O: u8> HSME_W<'a, O> {
    #[doc = "Disable High Speed Mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSME_A::_0)
    }
    #[doc = "Enable High Speed Mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSME_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn male(&self) -> MALE_R {
        MALE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn nale(&self) -> NALE_R {
        NALE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn sale(&self) -> SALE_R {
        SALE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SCL Synchronous Circuit Enable"]
    #[inline(always)]
    pub fn scsyne(&self) -> SCSYNE_R {
        SCSYNE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SMBus/I2C Bus Selection"]
    #[inline(always)]
    pub fn smbs(&self) -> SMBS_R {
        SMBS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast-mode Plus Enable"]
    #[inline(always)]
    pub fn fmpe(&self) -> FMPE_R {
        FMPE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - High Speed Mode Enable"]
    #[inline(always)]
    pub fn hsme(&self) -> HSME_R {
        HSME_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn male(&mut self) -> MALE_W<0> {
        MALE_W::new(self)
    }
    #[doc = "Bit 1 - NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nale(&mut self) -> NALE_W<1> {
        NALE_W::new(self)
    }
    #[doc = "Bit 2 - Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sale(&mut self) -> SALE_W<2> {
        SALE_W::new(self)
    }
    #[doc = "Bit 8 - SCL Synchronous Circuit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scsyne(&mut self) -> SCSYNE_W<8> {
        SCSYNE_W::new(self)
    }
    #[doc = "Bit 12 - SMBus/I2C Bus Selection"]
    #[inline(always)]
    #[must_use]
    pub fn smbs(&mut self) -> SMBS_W<12> {
        SMBS_W::new(self)
    }
    #[doc = "Bit 14 - Fast-mode Plus Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpe(&mut self) -> FMPE_W<14> {
        FMPE_W::new(self)
    }
    #[doc = "Bit 15 - High Speed Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsme(&mut self) -> HSME_W<15> {
        HSME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Function Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfctl](index.html) module"]
pub struct BFCTL_SPEC;
impl crate::RegisterSpec for BFCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bfctl::R](R) reader structure"]
impl crate::Readable for BFCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfctl::W](W) writer structure"]
impl crate::Writable for BFCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFCTL to value 0x0101"]
impl crate::Resettable for BFCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
