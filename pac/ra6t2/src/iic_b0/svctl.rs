#[doc = "Register `SVCTL` reader"]
pub struct R(crate::R<SVCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVCTL` writer"]
pub struct W(crate::W<SVCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVCTL_SPEC>;
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
impl From<crate::W<SVCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCAE` reader - General Call Address Enable"]
pub type GCAE_R = crate::BitReader<GCAE_A>;
#[doc = "General Call Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCAE_A {
    #[doc = "0: General call address detection disables."]
    _0 = 0,
    #[doc = "1: General call address detection enables."]
    _1 = 1,
}
impl From<GCAE_A> for bool {
    #[inline(always)]
    fn from(variant: GCAE_A) -> Self {
        variant as u8 != 0
    }
}
impl GCAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCAE_A {
        match self.bits {
            false => GCAE_A::_0,
            true => GCAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GCAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GCAE_A::_1
    }
}
#[doc = "Field `GCAE` writer - General Call Address Enable"]
pub type GCAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVCTL_SPEC, GCAE_A, O>;
impl<'a, const O: u8> GCAE_W<'a, O> {
    #[doc = "General call address detection disables."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCAE_A::_0)
    }
    #[doc = "General call address detection enables."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCAE_A::_1)
    }
}
#[doc = "Field `HSMCE` reader - Hs-mode Master Code Enable"]
pub type HSMCE_R = crate::BitReader<HSMCE_A>;
#[doc = "Hs-mode Master Code Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSMCE_A {
    #[doc = "0: Hs-mode Master Code Detection disables."]
    _0 = 0,
    #[doc = "1: Hs-mode Master Code Detection enables."]
    _1 = 1,
}
impl From<HSMCE_A> for bool {
    #[inline(always)]
    fn from(variant: HSMCE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSMCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSMCE_A {
        match self.bits {
            false => HSMCE_A::_0,
            true => HSMCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSMCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSMCE_A::_1
    }
}
#[doc = "Field `HSMCE` writer - Hs-mode Master Code Enable"]
pub type HSMCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVCTL_SPEC, HSMCE_A, O>;
impl<'a, const O: u8> HSMCE_W<'a, O> {
    #[doc = "Hs-mode Master Code Detection disables."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSMCE_A::_0)
    }
    #[doc = "Hs-mode Master Code Detection enables."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSMCE_A::_1)
    }
}
#[doc = "Field `DVIDE` reader - Device-ID Address Enable"]
pub type DVIDE_R = crate::BitReader<DVIDE_A>;
#[doc = "Device-ID Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVIDE_A {
    #[doc = "0: Device-ID address detection disables."]
    _0 = 0,
    #[doc = "1: Device-ID address detection enables."]
    _1 = 1,
}
impl From<DVIDE_A> for bool {
    #[inline(always)]
    fn from(variant: DVIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl DVIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVIDE_A {
        match self.bits {
            false => DVIDE_A::_0,
            true => DVIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVIDE_A::_1
    }
}
#[doc = "Field `DVIDE` writer - Device-ID Address Enable"]
pub type DVIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVCTL_SPEC, DVIDE_A, O>;
impl<'a, const O: u8> DVIDE_W<'a, O> {
    #[doc = "Device-ID address detection disables."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVIDE_A::_0)
    }
    #[doc = "Device-ID address detection enables."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVIDE_A::_1)
    }
}
#[doc = "Field `HOAE` reader - Host Address Enable"]
pub type HOAE_R = crate::BitReader<HOAE_A>;
#[doc = "Host Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOAE_A {
    #[doc = "0: Host address detection disables."]
    _0 = 0,
    #[doc = "1: Host address detection enables."]
    _1 = 1,
}
impl From<HOAE_A> for bool {
    #[inline(always)]
    fn from(variant: HOAE_A) -> Self {
        variant as u8 != 0
    }
}
impl HOAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOAE_A {
        match self.bits {
            false => HOAE_A::_0,
            true => HOAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOAE_A::_1
    }
}
#[doc = "Field `HOAE` writer - Host Address Enable"]
pub type HOAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVCTL_SPEC, HOAE_A, O>;
impl<'a, const O: u8> HOAE_W<'a, O> {
    #[doc = "Host address detection disables."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOAE_A::_0)
    }
    #[doc = "Host address detection enables."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOAE_A::_1)
    }
}
#[doc = "Field `SVAE0` reader - Slave Address Enable 0"]
pub type SVAE0_R = crate::BitReader<SVAE0_A>;
#[doc = "Slave Address Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVAE0_A {
    #[doc = "0: Slave 0 disables"]
    _0 = 0,
    #[doc = "1: Slave 0 enables"]
    _1 = 1,
}
impl From<SVAE0_A> for bool {
    #[inline(always)]
    fn from(variant: SVAE0_A) -> Self {
        variant as u8 != 0
    }
}
impl SVAE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVAE0_A {
        match self.bits {
            false => SVAE0_A::_0,
            true => SVAE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SVAE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVAE0_A::_1
    }
}
#[doc = "Field `SVAE0` writer - Slave Address Enable 0"]
pub type SVAE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVCTL_SPEC, SVAE0_A, O>;
impl<'a, const O: u8> SVAE0_W<'a, O> {
    #[doc = "Slave 0 disables"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVAE0_A::_0)
    }
    #[doc = "Slave 0 enables"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVAE0_A::_1)
    }
}
#[doc = "Field `SVAE1` reader - Slave Address Enable 1"]
pub type SVAE1_R = crate::BitReader<SVAE1_A>;
#[doc = "Slave Address Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVAE1_A {
    #[doc = "0: Slave 1 disables"]
    _0 = 0,
    #[doc = "1: Slave 1 enables"]
    _1 = 1,
}
impl From<SVAE1_A> for bool {
    #[inline(always)]
    fn from(variant: SVAE1_A) -> Self {
        variant as u8 != 0
    }
}
impl SVAE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVAE1_A {
        match self.bits {
            false => SVAE1_A::_0,
            true => SVAE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SVAE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVAE1_A::_1
    }
}
#[doc = "Field `SVAE1` writer - Slave Address Enable 1"]
pub type SVAE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVCTL_SPEC, SVAE1_A, O>;
impl<'a, const O: u8> SVAE1_W<'a, O> {
    #[doc = "Slave 1 disables"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVAE1_A::_0)
    }
    #[doc = "Slave 1 enables"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVAE1_A::_1)
    }
}
#[doc = "Field `SVAE2` reader - Slave Address Enable 2"]
pub type SVAE2_R = crate::BitReader<SVAE2_A>;
#[doc = "Slave Address Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVAE2_A {
    #[doc = "0: Slave 2 disables"]
    _0 = 0,
    #[doc = "1: Slave 2 enables"]
    _1 = 1,
}
impl From<SVAE2_A> for bool {
    #[inline(always)]
    fn from(variant: SVAE2_A) -> Self {
        variant as u8 != 0
    }
}
impl SVAE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVAE2_A {
        match self.bits {
            false => SVAE2_A::_0,
            true => SVAE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SVAE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVAE2_A::_1
    }
}
#[doc = "Field `SVAE2` writer - Slave Address Enable 2"]
pub type SVAE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVCTL_SPEC, SVAE2_A, O>;
impl<'a, const O: u8> SVAE2_W<'a, O> {
    #[doc = "Slave 2 disables"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVAE2_A::_0)
    }
    #[doc = "Slave 2 enables"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVAE2_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline(always)]
    pub fn gcae(&self) -> GCAE_R {
        GCAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Hs-mode Master Code Enable"]
    #[inline(always)]
    pub fn hsmce(&self) -> HSMCE_R {
        HSMCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Device-ID Address Enable"]
    #[inline(always)]
    pub fn dvide(&self) -> DVIDE_R {
        DVIDE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 15 - Host Address Enable"]
    #[inline(always)]
    pub fn hoae(&self) -> HOAE_R {
        HOAE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave Address Enable 0"]
    #[inline(always)]
    pub fn svae0(&self) -> SVAE0_R {
        SVAE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave Address Enable 1"]
    #[inline(always)]
    pub fn svae1(&self) -> SVAE1_R {
        SVAE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave Address Enable 2"]
    #[inline(always)]
    pub fn svae2(&self) -> SVAE2_R {
        SVAE2_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcae(&mut self) -> GCAE_W<0> {
        GCAE_W::new(self)
    }
    #[doc = "Bit 5 - Hs-mode Master Code Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsmce(&mut self) -> HSMCE_W<5> {
        HSMCE_W::new(self)
    }
    #[doc = "Bit 6 - Device-ID Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dvide(&mut self) -> DVIDE_W<6> {
        DVIDE_W::new(self)
    }
    #[doc = "Bit 15 - Host Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hoae(&mut self) -> HOAE_W<15> {
        HOAE_W::new(self)
    }
    #[doc = "Bit 16 - Slave Address Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn svae0(&mut self) -> SVAE0_W<16> {
        SVAE0_W::new(self)
    }
    #[doc = "Bit 17 - Slave Address Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn svae1(&mut self) -> SVAE1_W<17> {
        SVAE1_W::new(self)
    }
    #[doc = "Bit 18 - Slave Address Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn svae2(&mut self) -> SVAE2_W<18> {
        SVAE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svctl](index.html) module"]
pub struct SVCTL_SPEC;
impl crate::RegisterSpec for SVCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [svctl::R](R) reader structure"]
impl crate::Readable for SVCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svctl::W](W) writer structure"]
impl crate::Writable for SVCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SVCTL to value 0"]
impl crate::Resettable for SVCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
