#[doc = "Register `GTWP` reader"]
pub struct R(crate::R<GTWP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTWP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTWP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTWP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTWP` writer"]
pub struct W(crate::W<GTWP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTWP_SPEC>;
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
impl From<crate::W<GTWP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTWP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WP` reader - Register Write Disable"]
pub type WP_R = crate::BitReader<WP_A>;
#[doc = "Register Write Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_A {
    #[doc = "0: Write to the register enabled"]
    _0 = 0,
    #[doc = "1: Write to the register disabled"]
    _1 = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
impl WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::_0,
            true => WP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_A::_1
    }
}
#[doc = "Field `WP` writer - Register Write Disable"]
pub type WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTWP_SPEC, WP_A, O>;
impl<'a, const O: u8> WP_W<'a, O> {
    #[doc = "Write to the register enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP_A::_0)
    }
    #[doc = "Write to the register disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP_A::_1)
    }
}
#[doc = "Field `STRWP` reader - GTSTR.CSTRT Bit Write Disable"]
pub type STRWP_R = crate::BitReader<STRWP_A>;
#[doc = "GTSTR.CSTRT Bit Write Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRWP_A {
    #[doc = "0: Write to the bit is enabled"]
    _0 = 0,
    #[doc = "1: Write to the bit is disabled"]
    _1 = 1,
}
impl From<STRWP_A> for bool {
    #[inline(always)]
    fn from(variant: STRWP_A) -> Self {
        variant as u8 != 0
    }
}
impl STRWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRWP_A {
        match self.bits {
            false => STRWP_A::_0,
            true => STRWP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STRWP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STRWP_A::_1
    }
}
#[doc = "Field `STRWP` writer - GTSTR.CSTRT Bit Write Disable"]
pub type STRWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTWP_SPEC, STRWP_A, O>;
impl<'a, const O: u8> STRWP_W<'a, O> {
    #[doc = "Write to the bit is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STRWP_A::_0)
    }
    #[doc = "Write to the bit is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STRWP_A::_1)
    }
}
#[doc = "Field `STPWP` reader - GTSTP.CSTOP Bit Write Disable"]
pub type STPWP_R = crate::BitReader<STPWP_A>;
#[doc = "GTSTP.CSTOP Bit Write Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STPWP_A {
    #[doc = "0: Write to the bit is enabled"]
    _0 = 0,
    #[doc = "1: Write to the bit is disabled"]
    _1 = 1,
}
impl From<STPWP_A> for bool {
    #[inline(always)]
    fn from(variant: STPWP_A) -> Self {
        variant as u8 != 0
    }
}
impl STPWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPWP_A {
        match self.bits {
            false => STPWP_A::_0,
            true => STPWP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STPWP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STPWP_A::_1
    }
}
#[doc = "Field `STPWP` writer - GTSTP.CSTOP Bit Write Disable"]
pub type STPWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTWP_SPEC, STPWP_A, O>;
impl<'a, const O: u8> STPWP_W<'a, O> {
    #[doc = "Write to the bit is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STPWP_A::_0)
    }
    #[doc = "Write to the bit is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STPWP_A::_1)
    }
}
#[doc = "Field `CLRWP` reader - GTCLR.CCLR Bit Write Disable"]
pub type CLRWP_R = crate::BitReader<CLRWP_A>;
#[doc = "GTCLR.CCLR Bit Write Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRWP_A {
    #[doc = "0: Write to the bit is enabled"]
    _0 = 0,
    #[doc = "1: Write to the bit is disabled"]
    _1 = 1,
}
impl From<CLRWP_A> for bool {
    #[inline(always)]
    fn from(variant: CLRWP_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRWP_A {
        match self.bits {
            false => CLRWP_A::_0,
            true => CLRWP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLRWP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLRWP_A::_1
    }
}
#[doc = "Field `CLRWP` writer - GTCLR.CCLR Bit Write Disable"]
pub type CLRWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTWP_SPEC, CLRWP_A, O>;
impl<'a, const O: u8> CLRWP_W<'a, O> {
    #[doc = "Write to the bit is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLRWP_A::_0)
    }
    #[doc = "Write to the bit is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLRWP_A::_1)
    }
}
#[doc = "Field `CMNWP` reader - Common Register Write Disabled"]
pub type CMNWP_R = crate::BitReader<CMNWP_A>;
#[doc = "Common Register Write Disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMNWP_A {
    #[doc = "0: Write to the register is enabled"]
    _0 = 0,
    #[doc = "1: Write to the register is disabled"]
    _1 = 1,
}
impl From<CMNWP_A> for bool {
    #[inline(always)]
    fn from(variant: CMNWP_A) -> Self {
        variant as u8 != 0
    }
}
impl CMNWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMNWP_A {
        match self.bits {
            false => CMNWP_A::_0,
            true => CMNWP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMNWP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMNWP_A::_1
    }
}
#[doc = "Field `CMNWP` writer - Common Register Write Disabled"]
pub type CMNWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTWP_SPEC, CMNWP_A, O>;
impl<'a, const O: u8> CMNWP_W<'a, O> {
    #[doc = "Write to the register is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMNWP_A::_0)
    }
    #[doc = "Write to the register is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMNWP_A::_1)
    }
}
#[doc = "Field `PRKEY` writer - GTWP Key Code"]
pub type PRKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTWP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Register Write Disable"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GTSTR.CSTRT Bit Write Disable"]
    #[inline(always)]
    pub fn strwp(&self) -> STRWP_R {
        STRWP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GTSTP.CSTOP Bit Write Disable"]
    #[inline(always)]
    pub fn stpwp(&self) -> STPWP_R {
        STPWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GTCLR.CCLR Bit Write Disable"]
    #[inline(always)]
    pub fn clrwp(&self) -> CLRWP_R {
        CLRWP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Common Register Write Disabled"]
    #[inline(always)]
    pub fn cmnwp(&self) -> CMNWP_R {
        CMNWP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<0> {
        WP_W::new(self)
    }
    #[doc = "Bit 1 - GTSTR.CSTRT Bit Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn strwp(&mut self) -> STRWP_W<1> {
        STRWP_W::new(self)
    }
    #[doc = "Bit 2 - GTSTP.CSTOP Bit Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn stpwp(&mut self) -> STPWP_W<2> {
        STPWP_W::new(self)
    }
    #[doc = "Bit 3 - GTCLR.CCLR Bit Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn clrwp(&mut self) -> CLRWP_W<3> {
        CLRWP_W::new(self)
    }
    #[doc = "Bit 4 - Common Register Write Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmnwp(&mut self) -> CMNWP_W<4> {
        CMNWP_W::new(self)
    }
    #[doc = "Bits 8:15 - GTWP Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn prkey(&mut self) -> PRKEY_W<8> {
        PRKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Write-Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtwp](index.html) module"]
pub struct GTWP_SPEC;
impl crate::RegisterSpec for GTWP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtwp::R](R) reader structure"]
impl crate::Readable for GTWP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtwp::W](W) writer structure"]
impl crate::Writable for GTWP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTWP to value 0"]
impl crate::Resettable for GTWP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
