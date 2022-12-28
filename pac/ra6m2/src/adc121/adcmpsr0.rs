#[doc = "Register `ADCMPSR0` reader"]
pub struct R(crate::R<ADCMPSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMPSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMPSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMPSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMPSR0` writer"]
pub struct W(crate::W<ADCMPSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMPSR0_SPEC>;
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
impl From<crate::W<ADCMPSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMPSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPSTCHA00` reader - Compare window A flag of AN000\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTCHA00_R = crate::BitReader<CMPSTCHA00_A>;
#[doc = "Compare window A flag of AN000\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA00_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA00_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA00_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA00_A {
        match self.bits {
            false => CMPSTCHA00_A::_0,
            true => CMPSTCHA00_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA00_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA00_A::_1
    }
}
#[doc = "Field `CMPSTCHA00` writer - Compare window A flag of AN000"]
pub type CMPSTCHA00_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u16, ADCMPSR0_SPEC, CMPSTCHA00_A, O>;
impl<'a, const O: u8> CMPSTCHA00_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA00_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA00_A::_1)
    }
}
#[doc = "Field `CMPSTCHA01` reader - Compare window A flag of AN001\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTCHA01_R = crate::BitReader<CMPSTCHA01_A>;
#[doc = "Compare window A flag of AN001\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA01_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA01_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA01_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA01_A {
        match self.bits {
            false => CMPSTCHA01_A::_0,
            true => CMPSTCHA01_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA01_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA01_A::_1
    }
}
#[doc = "Field `CMPSTCHA01` writer - Compare window A flag of AN001"]
pub type CMPSTCHA01_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u16, ADCMPSR0_SPEC, CMPSTCHA01_A, O>;
impl<'a, const O: u8> CMPSTCHA01_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA01_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA01_A::_1)
    }
}
#[doc = "Field `CMPSTCHA02` reader - Compare window A flag of AN002\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTCHA02_R = crate::BitReader<CMPSTCHA02_A>;
#[doc = "Compare window A flag of AN002\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA02_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA02_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA02_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA02_A {
        match self.bits {
            false => CMPSTCHA02_A::_0,
            true => CMPSTCHA02_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA02_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA02_A::_1
    }
}
#[doc = "Field `CMPSTCHA02` writer - Compare window A flag of AN002"]
pub type CMPSTCHA02_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u16, ADCMPSR0_SPEC, CMPSTCHA02_A, O>;
impl<'a, const O: u8> CMPSTCHA02_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA02_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA02_A::_1)
    }
}
#[doc = "Field `CMPSTCHA03` reader - Compare window A flag of AN003\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTCHA03_R = crate::BitReader<CMPSTCHA03_A>;
#[doc = "Compare window A flag of AN003\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA03_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA03_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA03_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA03_A {
        match self.bits {
            false => CMPSTCHA03_A::_0,
            true => CMPSTCHA03_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA03_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA03_A::_1
    }
}
#[doc = "Field `CMPSTCHA03` writer - Compare window A flag of AN003"]
pub type CMPSTCHA03_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u16, ADCMPSR0_SPEC, CMPSTCHA03_A, O>;
impl<'a, const O: u8> CMPSTCHA03_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA03_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA03_A::_1)
    }
}
#[doc = "Field `CMPSTCHA05` reader - Compare window A flag of AN005\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTCHA05_R = crate::BitReader<CMPSTCHA05_A>;
#[doc = "Compare window A flag of AN005\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA05_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA05_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA05_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA05_A {
        match self.bits {
            false => CMPSTCHA05_A::_0,
            true => CMPSTCHA05_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA05_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA05_A::_1
    }
}
#[doc = "Field `CMPSTCHA05` writer - Compare window A flag of AN005"]
pub type CMPSTCHA05_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u16, ADCMPSR0_SPEC, CMPSTCHA05_A, O>;
impl<'a, const O: u8> CMPSTCHA05_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA05_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA05_A::_1)
    }
}
#[doc = "Field `CMPSTCHA06` reader - Compare window A flag of AN006\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTCHA06_R = crate::BitReader<CMPSTCHA06_A>;
#[doc = "Compare window A flag of AN006\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA06_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA06_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA06_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA06_A {
        match self.bits {
            false => CMPSTCHA06_A::_0,
            true => CMPSTCHA06_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA06_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA06_A::_1
    }
}
#[doc = "Field `CMPSTCHA06` writer - Compare window A flag of AN006"]
pub type CMPSTCHA06_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u16, ADCMPSR0_SPEC, CMPSTCHA06_A, O>;
impl<'a, const O: u8> CMPSTCHA06_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA06_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA06_A::_1)
    }
}
#[doc = "Field `CMPSTCHA07` reader - Compare window A flag of AN007\n\nThe field is **modified** in some way after a read operation."]
pub type CMPSTCHA07_R = crate::BitReader<CMPSTCHA07_A>;
#[doc = "Compare window A flag of AN007\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA07_A {
    #[doc = "0: Comparison conditions are not met."]
    _0 = 0,
    #[doc = "1: Comparison conditions are met."]
    _1 = 1,
}
impl From<CMPSTCHA07_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA07_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPSTCHA07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPSTCHA07_A {
        match self.bits {
            false => CMPSTCHA07_A::_0,
            true => CMPSTCHA07_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA07_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA07_A::_1
    }
}
#[doc = "Field `CMPSTCHA07` writer - Compare window A flag of AN007"]
pub type CMPSTCHA07_W<'a, const O: u8> =
    crate::BitWriter0C<'a, u16, ADCMPSR0_SPEC, CMPSTCHA07_A, O>;
impl<'a, const O: u8> CMPSTCHA07_W<'a, O> {
    #[doc = "Comparison conditions are not met."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPSTCHA07_A::_0)
    }
    #[doc = "Comparison conditions are met."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPSTCHA07_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare window A flag of AN000"]
    #[inline(always)]
    pub fn cmpstcha00(&self) -> CMPSTCHA00_R {
        CMPSTCHA00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare window A flag of AN001"]
    #[inline(always)]
    pub fn cmpstcha01(&self) -> CMPSTCHA01_R {
        CMPSTCHA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare window A flag of AN002"]
    #[inline(always)]
    pub fn cmpstcha02(&self) -> CMPSTCHA02_R {
        CMPSTCHA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare window A flag of AN003"]
    #[inline(always)]
    pub fn cmpstcha03(&self) -> CMPSTCHA03_R {
        CMPSTCHA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare window A flag of AN005"]
    #[inline(always)]
    pub fn cmpstcha05(&self) -> CMPSTCHA05_R {
        CMPSTCHA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare window A flag of AN006"]
    #[inline(always)]
    pub fn cmpstcha06(&self) -> CMPSTCHA06_R {
        CMPSTCHA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare window A flag of AN007"]
    #[inline(always)]
    pub fn cmpstcha07(&self) -> CMPSTCHA07_R {
        CMPSTCHA07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare window A flag of AN000"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha00(&mut self) -> CMPSTCHA00_W<0> {
        CMPSTCHA00_W::new(self)
    }
    #[doc = "Bit 1 - Compare window A flag of AN001"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha01(&mut self) -> CMPSTCHA01_W<1> {
        CMPSTCHA01_W::new(self)
    }
    #[doc = "Bit 2 - Compare window A flag of AN002"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha02(&mut self) -> CMPSTCHA02_W<2> {
        CMPSTCHA02_W::new(self)
    }
    #[doc = "Bit 3 - Compare window A flag of AN003"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha03(&mut self) -> CMPSTCHA03_W<3> {
        CMPSTCHA03_W::new(self)
    }
    #[doc = "Bit 5 - Compare window A flag of AN005"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha05(&mut self) -> CMPSTCHA05_W<5> {
        CMPSTCHA05_W::new(self)
    }
    #[doc = "Bit 6 - Compare window A flag of AN006"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha06(&mut self) -> CMPSTCHA06_W<6> {
        CMPSTCHA06_W::new(self)
    }
    #[doc = "Bit 7 - Compare window A flag of AN007"]
    #[inline(always)]
    #[must_use]
    pub fn cmpstcha07(&mut self) -> CMPSTCHA07_W<7> {
        CMPSTCHA07_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Compare Function Window A Channel Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmpsr0](index.html) module"]
pub struct ADCMPSR0_SPEC;
impl crate::RegisterSpec for ADCMPSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcmpsr0::R](R) reader structure"]
impl crate::Readable for ADCMPSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmpsr0::W](W) writer structure"]
impl crate::Writable for ADCMPSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xef;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMPSR0 to value 0"]
impl crate::Resettable for ADCMPSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
