#[doc = "Register `KRM` reader"]
pub struct R(crate::R<KRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KRM` writer"]
pub struct W(crate::W<KRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KRM_SPEC>;
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
impl From<crate::W<KRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KIMC0` reader - Key Interrupt Mode Control n"]
pub type KIMC0_R = crate::BitReader<KIMC0_A>;
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIMC0_A {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<KIMC0_A> for bool {
    #[inline(always)]
    fn from(variant: KIMC0_A) -> Self {
        variant as u8 != 0
    }
}
impl KIMC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIMC0_A {
        match self.bits {
            false => KIMC0_A::_0,
            true => KIMC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIMC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIMC0_A::_1
    }
}
#[doc = "Field `KIMC0` writer - Key Interrupt Mode Control n"]
pub type KIMC0_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KIMC0_A, O>;
impl<'a, const O: u8> KIMC0_W<'a, O> {
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIMC0_A::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIMC0_A::_1)
    }
}
#[doc = "Field `KIMC1` reader - Key Interrupt Mode Control n"]
pub type KIMC1_R = crate::BitReader<KIMC1_A>;
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIMC1_A {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<KIMC1_A> for bool {
    #[inline(always)]
    fn from(variant: KIMC1_A) -> Self {
        variant as u8 != 0
    }
}
impl KIMC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIMC1_A {
        match self.bits {
            false => KIMC1_A::_0,
            true => KIMC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIMC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIMC1_A::_1
    }
}
#[doc = "Field `KIMC1` writer - Key Interrupt Mode Control n"]
pub type KIMC1_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KIMC1_A, O>;
impl<'a, const O: u8> KIMC1_W<'a, O> {
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIMC1_A::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIMC1_A::_1)
    }
}
#[doc = "Field `KIMC2` reader - Key Interrupt Mode Control n"]
pub type KIMC2_R = crate::BitReader<KIMC2_A>;
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIMC2_A {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<KIMC2_A> for bool {
    #[inline(always)]
    fn from(variant: KIMC2_A) -> Self {
        variant as u8 != 0
    }
}
impl KIMC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIMC2_A {
        match self.bits {
            false => KIMC2_A::_0,
            true => KIMC2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIMC2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIMC2_A::_1
    }
}
#[doc = "Field `KIMC2` writer - Key Interrupt Mode Control n"]
pub type KIMC2_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KIMC2_A, O>;
impl<'a, const O: u8> KIMC2_W<'a, O> {
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIMC2_A::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIMC2_A::_1)
    }
}
#[doc = "Field `KIMC3` reader - Key Interrupt Mode Control n"]
pub type KIMC3_R = crate::BitReader<KIMC3_A>;
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIMC3_A {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<KIMC3_A> for bool {
    #[inline(always)]
    fn from(variant: KIMC3_A) -> Self {
        variant as u8 != 0
    }
}
impl KIMC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIMC3_A {
        match self.bits {
            false => KIMC3_A::_0,
            true => KIMC3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIMC3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIMC3_A::_1
    }
}
#[doc = "Field `KIMC3` writer - Key Interrupt Mode Control n"]
pub type KIMC3_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KIMC3_A, O>;
impl<'a, const O: u8> KIMC3_W<'a, O> {
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIMC3_A::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIMC3_A::_1)
    }
}
#[doc = "Field `KIMC4` reader - Key Interrupt Mode Control n"]
pub type KIMC4_R = crate::BitReader<KIMC4_A>;
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIMC4_A {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<KIMC4_A> for bool {
    #[inline(always)]
    fn from(variant: KIMC4_A) -> Self {
        variant as u8 != 0
    }
}
impl KIMC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIMC4_A {
        match self.bits {
            false => KIMC4_A::_0,
            true => KIMC4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIMC4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIMC4_A::_1
    }
}
#[doc = "Field `KIMC4` writer - Key Interrupt Mode Control n"]
pub type KIMC4_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KIMC4_A, O>;
impl<'a, const O: u8> KIMC4_W<'a, O> {
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIMC4_A::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIMC4_A::_1)
    }
}
#[doc = "Field `KIMC5` reader - Key Interrupt Mode Control n"]
pub type KIMC5_R = crate::BitReader<KIMC5_A>;
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIMC5_A {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<KIMC5_A> for bool {
    #[inline(always)]
    fn from(variant: KIMC5_A) -> Self {
        variant as u8 != 0
    }
}
impl KIMC5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIMC5_A {
        match self.bits {
            false => KIMC5_A::_0,
            true => KIMC5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIMC5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIMC5_A::_1
    }
}
#[doc = "Field `KIMC5` writer - Key Interrupt Mode Control n"]
pub type KIMC5_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KIMC5_A, O>;
impl<'a, const O: u8> KIMC5_W<'a, O> {
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIMC5_A::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIMC5_A::_1)
    }
}
#[doc = "Field `KIMC6` reader - Key Interrupt Mode Control n"]
pub type KIMC6_R = crate::BitReader<KIMC6_A>;
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIMC6_A {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<KIMC6_A> for bool {
    #[inline(always)]
    fn from(variant: KIMC6_A) -> Self {
        variant as u8 != 0
    }
}
impl KIMC6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIMC6_A {
        match self.bits {
            false => KIMC6_A::_0,
            true => KIMC6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIMC6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIMC6_A::_1
    }
}
#[doc = "Field `KIMC6` writer - Key Interrupt Mode Control n"]
pub type KIMC6_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KIMC6_A, O>;
impl<'a, const O: u8> KIMC6_W<'a, O> {
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIMC6_A::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIMC6_A::_1)
    }
}
#[doc = "Field `KIMC7` reader - Key Interrupt Mode Control n"]
pub type KIMC7_R = crate::BitReader<KIMC7_A>;
#[doc = "Key Interrupt Mode Control n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KIMC7_A {
    #[doc = "0: Do not detect key interrupt signals"]
    _0 = 0,
    #[doc = "1: Detect key interrupt signals"]
    _1 = 1,
}
impl From<KIMC7_A> for bool {
    #[inline(always)]
    fn from(variant: KIMC7_A) -> Self {
        variant as u8 != 0
    }
}
impl KIMC7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KIMC7_A {
        match self.bits {
            false => KIMC7_A::_0,
            true => KIMC7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KIMC7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KIMC7_A::_1
    }
}
#[doc = "Field `KIMC7` writer - Key Interrupt Mode Control n"]
pub type KIMC7_W<'a, const O: u8> = crate::BitWriter<'a, u8, KRM_SPEC, KIMC7_A, O>;
impl<'a, const O: u8> KIMC7_W<'a, O> {
    #[doc = "Do not detect key interrupt signals"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(KIMC7_A::_0)
    }
    #[doc = "Detect key interrupt signals"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(KIMC7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc0(&self) -> KIMC0_R {
        KIMC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc1(&self) -> KIMC1_R {
        KIMC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc2(&self) -> KIMC2_R {
        KIMC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc3(&self) -> KIMC3_R {
        KIMC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc4(&self) -> KIMC4_R {
        KIMC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc5(&self) -> KIMC5_R {
        KIMC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc6(&self) -> KIMC6_R {
        KIMC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Key Interrupt Mode Control n"]
    #[inline(always)]
    pub fn kimc7(&self) -> KIMC7_R {
        KIMC7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key Interrupt Mode Control n"]
    #[inline(always)]
    #[must_use]
    pub fn kimc0(&mut self) -> KIMC0_W<0> {
        KIMC0_W::new(self)
    }
    #[doc = "Bit 1 - Key Interrupt Mode Control n"]
    #[inline(always)]
    #[must_use]
    pub fn kimc1(&mut self) -> KIMC1_W<1> {
        KIMC1_W::new(self)
    }
    #[doc = "Bit 2 - Key Interrupt Mode Control n"]
    #[inline(always)]
    #[must_use]
    pub fn kimc2(&mut self) -> KIMC2_W<2> {
        KIMC2_W::new(self)
    }
    #[doc = "Bit 3 - Key Interrupt Mode Control n"]
    #[inline(always)]
    #[must_use]
    pub fn kimc3(&mut self) -> KIMC3_W<3> {
        KIMC3_W::new(self)
    }
    #[doc = "Bit 4 - Key Interrupt Mode Control n"]
    #[inline(always)]
    #[must_use]
    pub fn kimc4(&mut self) -> KIMC4_W<4> {
        KIMC4_W::new(self)
    }
    #[doc = "Bit 5 - Key Interrupt Mode Control n"]
    #[inline(always)]
    #[must_use]
    pub fn kimc5(&mut self) -> KIMC5_W<5> {
        KIMC5_W::new(self)
    }
    #[doc = "Bit 6 - Key Interrupt Mode Control n"]
    #[inline(always)]
    #[must_use]
    pub fn kimc6(&mut self) -> KIMC6_W<6> {
        KIMC6_W::new(self)
    }
    #[doc = "Bit 7 - Key Interrupt Mode Control n"]
    #[inline(always)]
    #[must_use]
    pub fn kimc7(&mut self) -> KIMC7_W<7> {
        KIMC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Return Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [krm](index.html) module"]
pub struct KRM_SPEC;
impl crate::RegisterSpec for KRM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [krm::R](R) reader structure"]
impl crate::Readable for KRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [krm::W](W) writer structure"]
impl crate::Writable for KRM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KRM to value 0"]
impl crate::Resettable for KRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
