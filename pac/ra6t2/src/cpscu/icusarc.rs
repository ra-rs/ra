#[doc = "Register `ICUSARC` reader"]
pub struct R(crate::R<ICUSARC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICUSARC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICUSARC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICUSARC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICUSARC` writer"]
pub struct W(crate::W<ICUSARC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICUSARC_SPEC>;
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
impl From<crate::W<ICUSARC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICUSARC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADMAC0` reader - Security attributes of registers for DMAC channel"]
pub type SADMAC0_R = crate::BitReader<SADMAC0_A>;
#[doc = "Security attributes of registers for DMAC channel\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADMAC0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SADMAC0_A> for bool {
    #[inline(always)]
    fn from(variant: SADMAC0_A) -> Self {
        variant as u8 != 0
    }
}
impl SADMAC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADMAC0_A {
        match self.bits {
            false => SADMAC0_A::_0,
            true => SADMAC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SADMAC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SADMAC0_A::_1
    }
}
#[doc = "Field `SADMAC0` writer - Security attributes of registers for DMAC channel"]
pub type SADMAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARC_SPEC, SADMAC0_A, O>;
impl<'a, const O: u8> SADMAC0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SADMAC0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SADMAC0_A::_1)
    }
}
#[doc = "Field `SADMAC1` reader - Security attributes of registers for DMAC channel"]
pub type SADMAC1_R = crate::BitReader<SADMAC1_A>;
#[doc = "Security attributes of registers for DMAC channel\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADMAC1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SADMAC1_A> for bool {
    #[inline(always)]
    fn from(variant: SADMAC1_A) -> Self {
        variant as u8 != 0
    }
}
impl SADMAC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADMAC1_A {
        match self.bits {
            false => SADMAC1_A::_0,
            true => SADMAC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SADMAC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SADMAC1_A::_1
    }
}
#[doc = "Field `SADMAC1` writer - Security attributes of registers for DMAC channel"]
pub type SADMAC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARC_SPEC, SADMAC1_A, O>;
impl<'a, const O: u8> SADMAC1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SADMAC1_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SADMAC1_A::_1)
    }
}
#[doc = "Field `SADMAC2` reader - Security attributes of registers for DMAC channel"]
pub type SADMAC2_R = crate::BitReader<SADMAC2_A>;
#[doc = "Security attributes of registers for DMAC channel\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADMAC2_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SADMAC2_A> for bool {
    #[inline(always)]
    fn from(variant: SADMAC2_A) -> Self {
        variant as u8 != 0
    }
}
impl SADMAC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADMAC2_A {
        match self.bits {
            false => SADMAC2_A::_0,
            true => SADMAC2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SADMAC2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SADMAC2_A::_1
    }
}
#[doc = "Field `SADMAC2` writer - Security attributes of registers for DMAC channel"]
pub type SADMAC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARC_SPEC, SADMAC2_A, O>;
impl<'a, const O: u8> SADMAC2_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SADMAC2_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SADMAC2_A::_1)
    }
}
#[doc = "Field `SADMAC3` reader - Security attributes of registers for DMAC channel"]
pub type SADMAC3_R = crate::BitReader<SADMAC3_A>;
#[doc = "Security attributes of registers for DMAC channel\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADMAC3_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SADMAC3_A> for bool {
    #[inline(always)]
    fn from(variant: SADMAC3_A) -> Self {
        variant as u8 != 0
    }
}
impl SADMAC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADMAC3_A {
        match self.bits {
            false => SADMAC3_A::_0,
            true => SADMAC3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SADMAC3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SADMAC3_A::_1
    }
}
#[doc = "Field `SADMAC3` writer - Security attributes of registers for DMAC channel"]
pub type SADMAC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARC_SPEC, SADMAC3_A, O>;
impl<'a, const O: u8> SADMAC3_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SADMAC3_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SADMAC3_A::_1)
    }
}
#[doc = "Field `SADMAC4` reader - Security attributes of registers for DMAC channel"]
pub type SADMAC4_R = crate::BitReader<SADMAC4_A>;
#[doc = "Security attributes of registers for DMAC channel\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADMAC4_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SADMAC4_A> for bool {
    #[inline(always)]
    fn from(variant: SADMAC4_A) -> Self {
        variant as u8 != 0
    }
}
impl SADMAC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADMAC4_A {
        match self.bits {
            false => SADMAC4_A::_0,
            true => SADMAC4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SADMAC4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SADMAC4_A::_1
    }
}
#[doc = "Field `SADMAC4` writer - Security attributes of registers for DMAC channel"]
pub type SADMAC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARC_SPEC, SADMAC4_A, O>;
impl<'a, const O: u8> SADMAC4_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SADMAC4_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SADMAC4_A::_1)
    }
}
#[doc = "Field `SADMAC5` reader - Security attributes of registers for DMAC channel"]
pub type SADMAC5_R = crate::BitReader<SADMAC5_A>;
#[doc = "Security attributes of registers for DMAC channel\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADMAC5_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SADMAC5_A> for bool {
    #[inline(always)]
    fn from(variant: SADMAC5_A) -> Self {
        variant as u8 != 0
    }
}
impl SADMAC5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADMAC5_A {
        match self.bits {
            false => SADMAC5_A::_0,
            true => SADMAC5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SADMAC5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SADMAC5_A::_1
    }
}
#[doc = "Field `SADMAC5` writer - Security attributes of registers for DMAC channel"]
pub type SADMAC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARC_SPEC, SADMAC5_A, O>;
impl<'a, const O: u8> SADMAC5_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SADMAC5_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SADMAC5_A::_1)
    }
}
#[doc = "Field `SADMAC6` reader - Security attributes of registers for DMAC channel"]
pub type SADMAC6_R = crate::BitReader<SADMAC6_A>;
#[doc = "Security attributes of registers for DMAC channel\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADMAC6_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SADMAC6_A> for bool {
    #[inline(always)]
    fn from(variant: SADMAC6_A) -> Self {
        variant as u8 != 0
    }
}
impl SADMAC6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADMAC6_A {
        match self.bits {
            false => SADMAC6_A::_0,
            true => SADMAC6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SADMAC6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SADMAC6_A::_1
    }
}
#[doc = "Field `SADMAC6` writer - Security attributes of registers for DMAC channel"]
pub type SADMAC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARC_SPEC, SADMAC6_A, O>;
impl<'a, const O: u8> SADMAC6_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SADMAC6_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SADMAC6_A::_1)
    }
}
#[doc = "Field `SADMAC7` reader - Security attributes of registers for DMAC channel"]
pub type SADMAC7_R = crate::BitReader<SADMAC7_A>;
#[doc = "Security attributes of registers for DMAC channel\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADMAC7_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SADMAC7_A> for bool {
    #[inline(always)]
    fn from(variant: SADMAC7_A) -> Self {
        variant as u8 != 0
    }
}
impl SADMAC7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADMAC7_A {
        match self.bits {
            false => SADMAC7_A::_0,
            true => SADMAC7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SADMAC7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SADMAC7_A::_1
    }
}
#[doc = "Field `SADMAC7` writer - Security attributes of registers for DMAC channel"]
pub type SADMAC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARC_SPEC, SADMAC7_A, O>;
impl<'a, const O: u8> SADMAC7_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SADMAC7_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SADMAC7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac0(&self) -> SADMAC0_R {
        SADMAC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac1(&self) -> SADMAC1_R {
        SADMAC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac2(&self) -> SADMAC2_R {
        SADMAC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac3(&self) -> SADMAC3_R {
        SADMAC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac4(&self) -> SADMAC4_R {
        SADMAC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac5(&self) -> SADMAC5_R {
        SADMAC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac6(&self) -> SADMAC6_R {
        SADMAC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    pub fn sadmac7(&self) -> SADMAC7_R {
        SADMAC7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn sadmac0(&mut self) -> SADMAC0_W<0> {
        SADMAC0_W::new(self)
    }
    #[doc = "Bit 1 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn sadmac1(&mut self) -> SADMAC1_W<1> {
        SADMAC1_W::new(self)
    }
    #[doc = "Bit 2 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn sadmac2(&mut self) -> SADMAC2_W<2> {
        SADMAC2_W::new(self)
    }
    #[doc = "Bit 3 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn sadmac3(&mut self) -> SADMAC3_W<3> {
        SADMAC3_W::new(self)
    }
    #[doc = "Bit 4 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn sadmac4(&mut self) -> SADMAC4_W<4> {
        SADMAC4_W::new(self)
    }
    #[doc = "Bit 5 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn sadmac5(&mut self) -> SADMAC5_W<5> {
        SADMAC5_W::new(self)
    }
    #[doc = "Bit 6 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn sadmac6(&mut self) -> SADMAC6_W<6> {
        SADMAC6_W::new(self)
    }
    #[doc = "Bit 7 - Security attributes of registers for DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn sadmac7(&mut self) -> SADMAC7_W<7> {
        SADMAC7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Controller Unit Security Attribution Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icusarc](index.html) module"]
pub struct ICUSARC_SPEC;
impl crate::RegisterSpec for ICUSARC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icusarc::R](R) reader structure"]
impl crate::Readable for ICUSARC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icusarc::W](W) writer structure"]
impl crate::Writable for ICUSARC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICUSARC to value 0xffff_ffff"]
impl crate::Resettable for ICUSARC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
