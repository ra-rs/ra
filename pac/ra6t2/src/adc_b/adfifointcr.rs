#[doc = "Register `ADFIFOINTCR` reader"]
pub struct R(crate::R<ADFIFOINTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOINTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOINTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOINTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADFIFOINTCR` writer"]
pub struct W(crate::W<ADFIFOINTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADFIFOINTCR_SPEC>;
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
impl From<crate::W<ADFIFOINTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADFIFOINTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOIE0` reader - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE0_R = crate::BitReader<FIFOIE0_A>;
#[doc = "Scan Group n FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOIE0_A {
    #[doc = "0: Disable scan group n FIFO interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO interrupt"]
    _1 = 1,
}
impl From<FIFOIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOIE0_A {
        match self.bits {
            false => FIFOIE0_A::_0,
            true => FIFOIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOIE0_A::_1
    }
}
#[doc = "Field `FIFOIE0` writer - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOINTCR_SPEC, FIFOIE0_A, O>;
impl<'a, const O: u8> FIFOIE0_W<'a, O> {
    #[doc = "Disable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOIE0_A::_0)
    }
    #[doc = "Enable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOIE0_A::_1)
    }
}
#[doc = "Field `FIFOIE1` reader - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE1_R = crate::BitReader<FIFOIE1_A>;
#[doc = "Scan Group n FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOIE1_A {
    #[doc = "0: Disable scan group n FIFO interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO interrupt"]
    _1 = 1,
}
impl From<FIFOIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOIE1_A {
        match self.bits {
            false => FIFOIE1_A::_0,
            true => FIFOIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOIE1_A::_1
    }
}
#[doc = "Field `FIFOIE1` writer - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOINTCR_SPEC, FIFOIE1_A, O>;
impl<'a, const O: u8> FIFOIE1_W<'a, O> {
    #[doc = "Disable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOIE1_A::_0)
    }
    #[doc = "Enable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOIE1_A::_1)
    }
}
#[doc = "Field `FIFOIE2` reader - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE2_R = crate::BitReader<FIFOIE2_A>;
#[doc = "Scan Group n FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOIE2_A {
    #[doc = "0: Disable scan group n FIFO interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO interrupt"]
    _1 = 1,
}
impl From<FIFOIE2_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOIE2_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOIE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOIE2_A {
        match self.bits {
            false => FIFOIE2_A::_0,
            true => FIFOIE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOIE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOIE2_A::_1
    }
}
#[doc = "Field `FIFOIE2` writer - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOINTCR_SPEC, FIFOIE2_A, O>;
impl<'a, const O: u8> FIFOIE2_W<'a, O> {
    #[doc = "Disable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOIE2_A::_0)
    }
    #[doc = "Enable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOIE2_A::_1)
    }
}
#[doc = "Field `FIFOIE3` reader - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE3_R = crate::BitReader<FIFOIE3_A>;
#[doc = "Scan Group n FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOIE3_A {
    #[doc = "0: Disable scan group n FIFO interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO interrupt"]
    _1 = 1,
}
impl From<FIFOIE3_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOIE3_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOIE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOIE3_A {
        match self.bits {
            false => FIFOIE3_A::_0,
            true => FIFOIE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOIE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOIE3_A::_1
    }
}
#[doc = "Field `FIFOIE3` writer - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOINTCR_SPEC, FIFOIE3_A, O>;
impl<'a, const O: u8> FIFOIE3_W<'a, O> {
    #[doc = "Disable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOIE3_A::_0)
    }
    #[doc = "Enable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOIE3_A::_1)
    }
}
#[doc = "Field `FIFOIE4` reader - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE4_R = crate::BitReader<FIFOIE4_A>;
#[doc = "Scan Group n FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOIE4_A {
    #[doc = "0: Disable scan group n FIFO interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO interrupt"]
    _1 = 1,
}
impl From<FIFOIE4_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOIE4_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOIE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOIE4_A {
        match self.bits {
            false => FIFOIE4_A::_0,
            true => FIFOIE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOIE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOIE4_A::_1
    }
}
#[doc = "Field `FIFOIE4` writer - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOINTCR_SPEC, FIFOIE4_A, O>;
impl<'a, const O: u8> FIFOIE4_W<'a, O> {
    #[doc = "Disable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOIE4_A::_0)
    }
    #[doc = "Enable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOIE4_A::_1)
    }
}
#[doc = "Field `FIFOIE5` reader - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE5_R = crate::BitReader<FIFOIE5_A>;
#[doc = "Scan Group n FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOIE5_A {
    #[doc = "0: Disable scan group n FIFO interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO interrupt"]
    _1 = 1,
}
impl From<FIFOIE5_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOIE5_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOIE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOIE5_A {
        match self.bits {
            false => FIFOIE5_A::_0,
            true => FIFOIE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOIE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOIE5_A::_1
    }
}
#[doc = "Field `FIFOIE5` writer - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOINTCR_SPEC, FIFOIE5_A, O>;
impl<'a, const O: u8> FIFOIE5_W<'a, O> {
    #[doc = "Disable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOIE5_A::_0)
    }
    #[doc = "Enable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOIE5_A::_1)
    }
}
#[doc = "Field `FIFOIE6` reader - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE6_R = crate::BitReader<FIFOIE6_A>;
#[doc = "Scan Group n FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOIE6_A {
    #[doc = "0: Disable scan group n FIFO interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO interrupt"]
    _1 = 1,
}
impl From<FIFOIE6_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOIE6_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOIE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOIE6_A {
        match self.bits {
            false => FIFOIE6_A::_0,
            true => FIFOIE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOIE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOIE6_A::_1
    }
}
#[doc = "Field `FIFOIE6` writer - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOINTCR_SPEC, FIFOIE6_A, O>;
impl<'a, const O: u8> FIFOIE6_W<'a, O> {
    #[doc = "Disable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOIE6_A::_0)
    }
    #[doc = "Enable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOIE6_A::_1)
    }
}
#[doc = "Field `FIFOIE7` reader - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE7_R = crate::BitReader<FIFOIE7_A>;
#[doc = "Scan Group n FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOIE7_A {
    #[doc = "0: Disable scan group n FIFO interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO interrupt"]
    _1 = 1,
}
impl From<FIFOIE7_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOIE7_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOIE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOIE7_A {
        match self.bits {
            false => FIFOIE7_A::_0,
            true => FIFOIE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOIE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOIE7_A::_1
    }
}
#[doc = "Field `FIFOIE7` writer - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOINTCR_SPEC, FIFOIE7_A, O>;
impl<'a, const O: u8> FIFOIE7_W<'a, O> {
    #[doc = "Disable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOIE7_A::_0)
    }
    #[doc = "Enable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOIE7_A::_1)
    }
}
#[doc = "Field `FIFOIE8` reader - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE8_R = crate::BitReader<FIFOIE8_A>;
#[doc = "Scan Group n FIFO Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOIE8_A {
    #[doc = "0: Disable scan group n FIFO interrupt"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO interrupt"]
    _1 = 1,
}
impl From<FIFOIE8_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOIE8_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOIE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOIE8_A {
        match self.bits {
            false => FIFOIE8_A::_0,
            true => FIFOIE8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOIE8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOIE8_A::_1
    }
}
#[doc = "Field `FIFOIE8` writer - Scan Group n FIFO Interrupt Enable"]
pub type FIFOIE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOINTCR_SPEC, FIFOIE8_A, O>;
impl<'a, const O: u8> FIFOIE8_W<'a, O> {
    #[doc = "Disable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOIE8_A::_0)
    }
    #[doc = "Enable scan group n FIFO interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOIE8_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn fifoie0(&self) -> FIFOIE0_R {
        FIFOIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn fifoie1(&self) -> FIFOIE1_R {
        FIFOIE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn fifoie2(&self) -> FIFOIE2_R {
        FIFOIE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn fifoie3(&self) -> FIFOIE3_R {
        FIFOIE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn fifoie4(&self) -> FIFOIE4_R {
        FIFOIE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn fifoie5(&self) -> FIFOIE5_R {
        FIFOIE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn fifoie6(&self) -> FIFOIE6_R {
        FIFOIE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn fifoie7(&self) -> FIFOIE7_R {
        FIFOIE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn fifoie8(&self) -> FIFOIE8_R {
        FIFOIE8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoie0(&mut self) -> FIFOIE0_W<0> {
        FIFOIE0_W::new(self)
    }
    #[doc = "Bit 1 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoie1(&mut self) -> FIFOIE1_W<1> {
        FIFOIE1_W::new(self)
    }
    #[doc = "Bit 2 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoie2(&mut self) -> FIFOIE2_W<2> {
        FIFOIE2_W::new(self)
    }
    #[doc = "Bit 3 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoie3(&mut self) -> FIFOIE3_W<3> {
        FIFOIE3_W::new(self)
    }
    #[doc = "Bit 4 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoie4(&mut self) -> FIFOIE4_W<4> {
        FIFOIE4_W::new(self)
    }
    #[doc = "Bit 5 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoie5(&mut self) -> FIFOIE5_W<5> {
        FIFOIE5_W::new(self)
    }
    #[doc = "Bit 6 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoie6(&mut self) -> FIFOIE6_W<6> {
        FIFOIE6_W::new(self)
    }
    #[doc = "Bit 7 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoie7(&mut self) -> FIFOIE7_W<7> {
        FIFOIE7_W::new(self)
    }
    #[doc = "Bit 8 - Scan Group n FIFO Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoie8(&mut self) -> FIFOIE8_W<8> {
        FIFOIE8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifointcr](index.html) module"]
pub struct ADFIFOINTCR_SPEC;
impl crate::RegisterSpec for ADFIFOINTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifointcr::R](R) reader structure"]
impl crate::Readable for ADFIFOINTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adfifointcr::W](W) writer structure"]
impl crate::Writable for ADFIFOINTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADFIFOINTCR to value 0"]
impl crate::Resettable for ADFIFOINTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
